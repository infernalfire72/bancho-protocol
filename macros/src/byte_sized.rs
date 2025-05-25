use crate::tools::FieldInfo;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{DataStruct, DeriveInput};

pub fn byte_sized_struct_impl(
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
                    self.#ident.as_ref().unwrap().byte_size()
                } else { 0 }
            }),
            None => quote_spanned!(f.span => {
                self.#ident.byte_size()
            }),
        }
    });

    quote! {
        impl #impl_generics #crate_root::serde::byte_sized::ByteSized for #ident #ty_generics #where_clause {
            fn byte_size(&self) -> usize {
                0#(+#field_impls)*
            }
        }
    }
}

pub fn byte_sized_enum_impl(
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
        impl #crate_root::serde::byte_sized::ByteSized for #ident {
            fn byte_size(&self) -> usize {
                <#repr>::byte_size(&unsafe { *((self as *const _) as *const #repr) })
            }
        }
    }
}
