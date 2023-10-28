use crate::tools::FieldInfo;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{DataStruct, DeriveInput};

pub fn binde_struct_impl(crate_root: TokenStream2, ident: &Ident, input: &DataStruct) -> TokenStream2 {
    let fields: Vec<FieldInfo> = FieldInfo::from(&input.fields);
    let field_idents = fields.iter().map(|f| &f.ident);
    let field_impls = fields.iter().map(|f| {
        let ident = &f.ident;
        match &f.depends_on {
            Some(depends_on) => quote_spanned! {f.span =>
                let #ident = match #depends_on {
                    true => Some(#crate_root::serde::deserialize::BinaryDeserialize::read_from(reader)?),
                    false => None,
                };
            },
            None => quote_spanned! {f.span =>
                let #ident = #crate_root::serde::deserialize::BinaryDeserialize::read_from(reader)?;
            }
        }
    });



    quote!{
        impl #crate_root::serde::deserialize::BinaryDeserialize for #ident {
            fn read_from(reader: &mut #crate_root::serde::ByteStream) -> std::io::Result<Self> where Self: Sized {
                #(#field_impls)*
                Ok(Self {
                  #(#field_idents),*
                })
            }
        }
    }
}

pub fn binde_enum_impl(crate_root: TokenStream2, ident: &Ident, input: &DeriveInput) -> TokenStream2 {
    let repr = input.attrs.iter().find(|attr| attr.path().is_ident("repr"));
    if repr.is_none() {
        return quote!{compile_error!("no repr attribute found on enum");};
    }
    let repr = repr.unwrap().parse_args::<TokenStream2>().unwrap();

    quote! {
        impl #crate_root::serde::deserialize::BinaryDeserialize for #ident {
            fn read_from(reader: &mut #crate_root::serde::ByteStream) -> std::io::Result<Self> where Self: Sized {
                Ok(unsafe { std::mem::transmute(<#repr>::read_from(reader)?) })
            }
        }
    }
}