use std::result;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Expr,
    ExprLit, Field, Fields, FieldsNamed, FieldsUnnamed, Index, Lit, LitStr, Meta, MetaNameValue,
    Token,
};

// #[heap_size]
const HEAP_IDENT: &str = "heap_size";
// #[heap_size(with = "...")] Field attributes
const HEAP_ATTR_WITH_IDENT: &str = "with";

#[proc_macro_derive(Heap, attributes(heap_size))]
pub fn heap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let tokens = match input.data {
        Data::Struct(..) => render_struct(input),
        // Data::Enum(..) => render_enum(input),
        Data::Enum(..) => unimplemented!("HeapSize can not be derived for an enum"),
        Data::Union(..) => unimplemented!("HeapSize can not be derived for a union"),
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
}

impl HeapAttr {
    fn new<T: ToTokens>(
        raw_attrs: &[Attribute],
        is_field: bool,
        origin: T,
    ) -> Result<Option<Self>> {
        let mut attrs = vec![];
        for attr in raw_attrs {
            if let Meta::List(meta_list) = &attr.meta {
                if meta_list.path.is_ident(HEAP_IDENT) {
                    let heap_attrs = meta_list
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                    if heap_attrs.len() > 1 {
                        bail!(meta_list, "too many heap_size attributes");
                    }
                    attrs.extend(heap_attrs);
                }
            } else {
                attrs.push(attr.meta.clone());
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

struct HeapField {
    attr: HeapAttr,
    ident: TokenStream,
    field: Field,
}

impl HeapField {
    fn new(index: usize, field: Field, container_attr: Option<&HeapAttr>) -> Result<Option<Self>> {
        let attr = match HeapAttr::new(&field.attrs, true, &field)? {
            Some(attr) => attr,
            None => {
                if let Some(HeapAttr::Container(ref meta)) = container_attr {
                    HeapAttr::Field(meta.clone())
                } else {
                    return Ok(None);
                }
            }
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

    fn method_heap_size(&self) -> Result<TokenStream> {
        let ident = &self.ident;
        match self.attr {
            HeapAttr::Field(_) => Ok(quote_spanned! {self.field.span()=>
                ::heapuse::HeapSize::heap_size(&self.#ident)
            }),
            HeapAttr::FieldWith(ref meta, ref mod_path) => {
                let path = syn::parse_str::<syn::Path>(&mod_path.value())?;
                Ok(quote_spanned! {meta.span()=>
                    #path::heap_size(&self.#ident)
                })
            }
            HeapAttr::Container(ref meta) => {
                bail!(
                    self.field.clone(),
                    "unexpected container attribute is found on field allowed in field: {}",
                    meta.to_token_stream().to_string()
                );
            }
        }
    }
}

fn render_struct(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let container_attrs = HeapAttr::new(&input.attrs, false, &input)?;

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
    for (i, field) in fields.into_iter().enumerate() {
        if let Some(f) = HeapField::new(i, field.clone(), container_attrs.as_ref())? {
            heap_sizes.push(f.method_heap_size()?);
        }
    }

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::heapuse::HeapSize for #ident #ty_generics #where_clause {
            fn heap_size(&self) -> usize {
                0 #(+ #heap_sizes)*
            }
        }
    })
}

// fn render_enum(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
// }
