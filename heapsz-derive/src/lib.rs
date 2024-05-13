#![doc = include_str!("../README.md")]

use std::result;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Expr,
    ExprLit, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Lit, LitStr, Meta,
    MetaNameValue, Token, Variant,
};

// #[heap_size]
const HEAP_IDENT: &str = "heap_size";
// #[heap_size(with = "...")] Field attributes
const HEAP_ATTR_WITH_IDENT: &str = "with";
// #[heap_size(skip)] Field attributes
const HEAP_ATTR_SKIP_IDENT: &str = "skip";

#[proc_macro_derive(HeapSize, attributes(heap_size))]
pub fn heap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let tokens = match input.data {
        Data::Struct(..) => render_struct(input),
        Data::Enum(..) => render_enum(input),
        Data::Union(..) => Err(syn::Error::new_spanned(
            input,
            "`Heap` can not be derived for a union",
        )),
    };
    tokens.unwrap_or_else(|e| e.into_compile_error()).into()
}

type Result<T> = result::Result<T, syn::Error>;
macro_rules! bail {
    ($token:expr, $($arg:tt)+) => {{
        return Err(syn::Error::new_spanned($token, format!($($arg)*)))
    }};
}

enum HeapAttr {
    Container(Meta),
    Field(Meta),
    FieldWith(Meta, LitStr),
    FieldSkip(Meta),
}

impl HeapAttr {
    fn new<T: ToTokens>(
        raw_attrs: &[Attribute],
        is_field: bool,
        is_variant: bool,
        origin: T,
    ) -> Result<Option<Self>> {
        let mut attrs = vec![];
        for attr in raw_attrs {
            match &attr.meta {
                Meta::List(meta_list) => {
                    if meta_list.path.is_ident(HEAP_IDENT) {
                        let heap_attrs = meta_list
                            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                        if heap_attrs.len() > 1 {
                            bail!(meta_list, "too many heap_size attributes");
                        }
                        attrs.extend(heap_attrs);
                    }
                }
                Meta::Path(path) => {
                    if path.is_ident(HEAP_IDENT) {
                        attrs.push(attr.meta.clone());
                    }
                }
                _ => (),
            }
        }
        let meta = if attrs.is_empty() {
            return Ok(None);
        } else if attrs.len() == 1 {
            attrs.pop().unwrap()
        } else {
            bail!(origin, "too many heap_size attributes")
        };

        match meta {
            Meta::Path(ref name) => {
                if name.is_ident(HEAP_IDENT) {
                    if is_field {
                        Ok(Some(HeapAttr::Field(meta)))
                    } else {
                        Ok(Some(HeapAttr::Container(meta)))
                    }
                } else if name.is_ident(HEAP_ATTR_SKIP_IDENT) {
                    if is_field || is_variant {
                        Ok(Some(HeapAttr::FieldSkip(meta)))
                    } else {
                        bail!(meta, "`#[heap_size(skip)]` is a field attribute")
                    }
                } else if name.is_ident(HEAP_ATTR_WITH_IDENT) {
                    bail!(
                        meta,
                        "heap_size attribute `with` must be followed by \
                        a module path, `with = \"some::mod\"`"
                    )
                } else {
                    let name = name.to_token_stream().to_string().replace(' ', "");
                    bail!(meta, "unknown heap_size attribute `{}`", name)
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(ref mod_path),
                        ..
                    }),
                ..
            }) => {
                if path.is_ident(HEAP_ATTR_WITH_IDENT) {
                    Ok(Some(HeapAttr::FieldWith(meta.clone(), mod_path.clone())))
                } else {
                    let name = path.to_token_stream().to_string().replace(' ', "");
                    bail!(meta, "unknown heap_size attribute `{}`", name)
                }
            }
            meta => {
                let full = meta.to_token_stream().to_string();
                bail!(meta, "unknown heap attribute `{}`", full)
            }
        }
    }
}

enum MethodReceiver {
    FieldIdent,
    Replace(Ident),
    PrefixRef(Ident),
}

struct HeapField {
    attr: HeapAttr,
    ident: TokenStream,
    field: Field,
}

impl HeapField {
    fn new(
        index: usize,
        field: Field,
        container_attr: Option<&HeapAttr>,
        variant_attr: Option<&HeapAttr>,
    ) -> Result<Option<Self>> {
        let require_container_attr = |meta| {
            if let Some(HeapAttr::Container(_)) = container_attr {
                Ok(None)
            } else {
                bail!(
                    meta,
                    "`#[heap_size(skip)]` is only allow with a container \
                    attribute `#[heap_size]`."
                );
            }
        };
        let attr = match HeapAttr::new(&field.attrs, true, false, &field)? {
            None => {
                if let Some(HeapAttr::FieldSkip(meta)) = variant_attr {
                    return require_container_attr(meta);
                } else if let Some(HeapAttr::Container(ref meta)) = container_attr {
                    HeapAttr::Field(meta.clone())
                } else {
                    return Ok(None);
                }
            }
            Some(HeapAttr::FieldSkip(meta)) => return require_container_attr(&meta),
            Some(attr) => attr,
        };

        let ident = field.ident.clone().map(|x| quote!(#x)).unwrap_or_else(|| {
            let index = Index {
                index: index as u32,
                span: Span::call_site(),
            };
            quote!(#index)
        });

        Ok(Some(HeapField { attr, ident, field }))
    }

    fn method_heap_size(&self, self_: &MethodReceiver) -> Result<TokenStream> {
        let field_ident = &self.ident;
        let ident = match self_ {
            MethodReceiver::FieldIdent => {
                quote_spanned!(self.field.span()=> #field_ident)
            }
            MethodReceiver::Replace(ident) => quote_spanned!(self.field.span()=> #ident),
            MethodReceiver::PrefixRef(ident) => {
                quote_spanned!(self.field.span()=> &#ident.#field_ident)
            }
        };
        match self.attr {
            HeapAttr::Field(_) => Ok(quote_spanned! {self.field.span()=>
                ::heapsz::HeapSize::heap_size(#ident)
            }),
            HeapAttr::FieldWith(ref meta, ref mod_path) => {
                let path = syn::parse_str::<syn::Path>(&mod_path.value())?;
                Ok(quote_spanned! {meta.span()=>
                    #path::heap_size(#ident)
                })
            }
            HeapAttr::FieldSkip(_) => {
                bail!(
                    self.field.clone(),
                    "internal error `#[heap_size(skip)]` field generates `fn heap_size()`",
                );
            }
            HeapAttr::Container(ref meta) => {
                bail!(
                    self.field.clone(),
                    "internal error unexpected container attribute is found on field: {}",
                    meta.to_token_stream().to_string()
                );
            }
        }
    }
}

fn render_struct(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let container_attrs = HeapAttr::new(&input.attrs, false, false, &input)?;

    let ident = input.ident.clone();
    let Data::Struct(data) = input.data else {
        bail!(input, "{} should be a struct", ident);
    };
    let fields = match data {
        DataStruct {
            fields: Fields::Named(FieldsNamed { named: fields, .. }),
            ..
        } => fields.into_iter().collect(),
        DataStruct {
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }),
            ..
        } => fields.into_iter().collect(),
        DataStruct {
            fields: Fields::Unit,
            ..
        } => Vec::new(),
    };

    let mut heap_sizes = vec![];
    let self_ = MethodReceiver::PrefixRef(Ident::new("self", Span::call_site()));
    for (i, field) in fields.into_iter().enumerate() {
        if let Some(f) = HeapField::new(i, field.clone(), container_attrs.as_ref(), None)? {
            heap_sizes.push(f.method_heap_size(&self_)?);
        }
    }

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::heapsz::HeapSize for #ident #ty_generics #where_clause {
            fn heap_size(&self) -> usize {
                0 #(+ #heap_sizes)*
            }
        }
    })
}

fn render_enum(input: DeriveInput) -> Result<TokenStream> {
    let container_attrs = HeapAttr::new(&input.attrs, false, false, &input)?;

    let ident = input.ident.clone();
    let Data::Enum(data) = input.data else {
        bail!(input, "{} should be an enum", ident);
    };
    let mut rendered_vars = vec![];
    for var in data.variants {
        rendered_vars.push(render_enum_variant(var, container_attrs.as_ref())?);
    }
    let matches = if rendered_vars.is_empty() {
        quote!(0)
    } else {
        quote! {
            #[allow(unused_variables)]
            match self {
                #(#rendered_vars)*
            }
        }
    };

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::heapsz::HeapSize for #ident #ty_generics #where_clause {
            fn heap_size(&self) -> usize {
                #matches
            }
        }
    })
}

fn render_enum_variant(var: Variant, container_attr: Option<&HeapAttr>) -> Result<TokenStream> {
    let var_attrs = HeapAttr::new(&var.attrs, false, true, &var)?;
    let var_span = var.span();
    let var_ident = var.ident;
    let (match_arm, self_receivers, fields) = match var.fields {
        Fields::Named(FieldsNamed { named: fields, .. }) => {
            let idents = fields.iter().map(|f| f.ident.clone().unwrap());
            let match_arm = quote_spanned! {var_span=>
                Self::#var_ident { #(#idents,)* }
            };
            let self_receivers = fields
                .iter()
                .map(|_| MethodReceiver::FieldIdent)
                .collect::<Vec<_>>();
            (
                match_arm,
                self_receivers,
                fields.into_iter().collect::<Vec<_>>(),
            )
        }
        Fields::Unnamed(FieldsUnnamed {
            unnamed: fields, ..
        }) => {
            let field_idents = fields
                .iter()
                .enumerate()
                .map(|(i, f)| Ident::new(&format!("f_{}", i), f.span()))
                .collect::<Vec<_>>();
            let self_receivers = field_idents
                .iter()
                .map(|ident| MethodReceiver::Replace(ident.clone()))
                .collect::<Vec<_>>();
            let match_arm = quote_spanned! {var_span=>
                Self::#var_ident(#(#field_idents,)*)
            };
            (
                match_arm,
                self_receivers,
                fields.into_iter().collect::<Vec<_>>(),
            )
        }
        Fields::Unit => {
            let match_arm = quote_spanned! {var_span=>
                Self::#var_ident
            };
            (match_arm, vec![], vec![])
        }
    };

    let mut heap_sizes = vec![];
    for (i, field) in fields.into_iter().enumerate() {
        if let Some(f) = HeapField::new(i, field.clone(), container_attr, var_attrs.as_ref())? {
            heap_sizes.push(f.method_heap_size(&self_receivers[i])?);
        }
    }

    Ok(quote_spanned! {var_span=>
        #match_arm => { 0 #(+ #heap_sizes)* }
    })
}
