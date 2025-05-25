use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Fields};

pub fn get_crate_root(attrs: &Vec<Attribute>) -> TokenStream2 {
    match attrs.iter().find(|a| a.path().is_ident("crate_root")) {
        Some(attr) => attr.parse_args::<proc_macro2::TokenStream>().unwrap(),
        None => "bancho_protocol".parse().unwrap(),
    }
}

pub struct FieldInfo {
    pub span: Span,
    pub ident: TokenStream2,
    pub depends_on: Option<TokenStream2>,
}

impl FieldInfo {
    pub fn from(fields: &Fields) -> Vec<Self> {
        let i = 0;
        let fields = match fields {
            Fields::Named(named) => named.named.iter(),
            Fields::Unnamed(unnamed) => unnamed.unnamed.iter(),
            Fields::Unit => return vec![],
        };

        fields
            .map(|f| {
                let span = f.span();
                let ident = match &f.ident {
                    Some(ident) => quote! {#ident},
                    None => format!("{}", i).parse().unwrap(),
                };

                let depends_on = match f.attrs.iter().find(|attr| attr.path().is_ident("depends")) {
                    Some(attr) => Some(attr.parse_args().unwrap()),
                    None => None,
                };

                FieldInfo {
                    span,
                    ident,
                    depends_on,
                }
            })
            .collect()
    }
}
