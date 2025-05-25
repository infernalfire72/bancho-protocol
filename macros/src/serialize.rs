use crate::tools::FieldInfo;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{DataStruct, DeriveInput};

pub fn binser_struct_impl(
    crate_root: TokenStream2,
    ident: &Ident,
    input: &DeriveInput,
    data: &DataStruct,
) -> TokenStream2 {
    let fields: Vec<FieldInfo> = FieldInfo::from(&data.fields);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_impls = fields.iter().map(|f| {
        let ident = &f.ident;
        match &f.depends_on {
            Some(depends_on) => quote_spanned!(f.span => {
                if self.#depends_on {
                    self.#ident.as_ref().unwrap().write_to(writer);
                }
            }),
            None => quote_spanned!(f.span => {
                self.#ident.write_to(writer);
            }),
        }
    });

    quote! {
        impl #impl_generics #crate_root::serde::serialize::BinarySerialize for #ident #ty_generics #where_clause {
            fn write_to(&self, writer: &mut #crate_root::serde::serialize::BinaryWriter) {
                #(#field_impls)*
            }
        }
    }
}

pub fn binser_enum_impl(
    crate_root: TokenStream2,
    ident: &Ident,
    input: &DeriveInput,
) -> TokenStream2 {
    let repr = input.attrs.iter().find(|attr| attr.path().is_ident("repr"));
    if repr.is_none() {
        return quote! {compile_error!("no repr attribute found on enum");};
    }
    let repr = repr.unwrap().parse_args::<TokenStream2>().unwrap();

    quote! {
        impl #crate_root::serde::serialize::BinarySerialize for #ident {
            fn write_to(&self, writer: &mut #crate_root::serde::serialize::BinaryWriter) {
                <#repr>::write_to(&unsafe { *((self as *const _) as *const #repr) }, writer)
            }
        }
    }
}
