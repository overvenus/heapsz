use std::result;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Data, DataStruct, DeriveInput, Expr, ExprLit, Field,
    Fields, FieldsNamed, FieldsUnnamed, Index, Lit, LitStr, Meta, MetaNameValue, Token,
};

// #[heap(...)]
const HEAP_IDENT: &str = "heap";
// #[heap(all)] Container attributes
#[allow(dead_code)]
const HEAP_ATTR_ALL_IDENT: &str = "all";
// #[heap(add)] Field attributes
const HEAP_ATTR_ADD_IDENT: &str = "add";
// #[heap(with = "...")] Field attributes
const HEAP_ATTR_WITH_IDENT: &str = "with";

#[proc_macro_derive(Heap, attributes(heap))]
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
    Add,
    With(LitStr), // TODO: Calculate heap size using the specified mod path.
}

impl HeapAttr {
    fn new(meta: Meta) -> Result<Self> {
        match meta {
            Meta::Path(ref name) => {
                if name.is_ident(HEAP_ATTR_ADD_IDENT) {
                    Ok(HeapAttr::Add)
                } else if name.is_ident(HEAP_ATTR_WITH_IDENT) {
                    bail!(
                        meta,
                        "heap attribute `with` miss mod path, \
                        it should be `with =\"your::mod\"`"
                    )
                } else {
                    let name = name.to_token_stream().to_string().replace(' ', "");
                    bail!(meta, "unknown heap attribute `{}`", name)
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
                if path.is_ident(HEAP_ATTR_ADD_IDENT) {
                    bail!(
                        meta,
                        "heap attribute `add` is followed by an unexpected mod path"
                    )
                } else if path.is_ident(HEAP_ATTR_WITH_IDENT) {
                    Ok(HeapAttr::With(mod_path.clone()))
                } else {
                    let name = path.to_token_stream().to_string().replace(' ', "");
                    bail!(meta, "unknown heap attribute `{}`", name)
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
    fn new(index: usize, field: Field) -> Result<Option<Self>> {
        let mut attrs = vec![];
        for attr in &field.attrs {
            if let Meta::List(meta_list) = &attr.meta {
                if meta_list.path.is_ident(HEAP_IDENT) {
                    let heap_attrs = meta_list
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                    if heap_attrs.len() > 1 {
                        bail!(meta_list, "too many heap attributes");
                    }

                    for m in heap_attrs {
                        attrs.push(HeapAttr::new(m)?)
                    }
                }
            } else {
                bail!(
                    attr,
                    "unsupported heap attribute, maybe you mean `#[heap(add)]`?"
                );
            }
        }
        if attrs.is_empty() {
            return Ok(None);
        } else if attrs.len() > 1 {
            bail!(field, "too many heap attributes")
        }

        let ident = field.ident.clone().map(|x| quote!(#x)).unwrap_or_else(|| {
            let index = Index {
                index: index as u32,
                span: Span::call_site(),
            };
            quote!(#index)
        });

        Ok(Some(HeapField {
            attr: attrs.pop().unwrap(),
            ident,
            field,
        }))
    }

    fn approximate_heap_size(&self) -> TokenStream {
        match self.attr {
            HeapAttr::Add => {
                let ident = &self.ident;
                quote_spanned! {self.field.span()=>
                    ::heapuse::HeapSize::approximate_heap_size(&self.#ident)
                }
            }
            HeapAttr::With(ref mod_path) => {
                unimplemented!("heap(with = \"{:?}\")", mod_path)
            }
        }
    }
}

fn render_struct(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
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

    // fields and their name.
    let mut heap_fields = vec![];
    for (i, field) in fields.into_iter().enumerate() {
        if let Some(f) = HeapField::new(i, field.clone())? {
            heap_fields.push(f);
        }
    }

    let approximate_heap_sizes = heap_fields.iter().map(|f| f.approximate_heap_size());

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::heapuse::HeapSize for #ident #ty_generics #where_clause {
            fn approximate_heap_size(&self) -> usize {
                0 #(+ #approximate_heap_sizes)*
            }
        }
    })
}

// fn render_enum(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
// }
