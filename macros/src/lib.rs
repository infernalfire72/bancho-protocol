mod byte_sized;
mod deserialize;
mod serialize;
mod tools;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, ItemStruct};

#[proc_macro_derive(BinarySerialize, attributes(crate_root, depends))]
pub fn binary_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let crate_root = tools::get_crate_root(&input.attrs);

    TokenStream::from(match input.data {
        Data::Struct(ref struct_data) => {
            serialize::binser_struct_impl(crate_root, ident, &input, struct_data)
        }
        Data::Enum(ref _enum_data) => serialize::binser_enum_impl(crate_root, ident, &input),
        _ => unimplemented!(),
    })
}

#[proc_macro_derive(BinaryDeserialize, attributes(crate_root, depends))]
pub fn binary_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let crate_root = tools::get_crate_root(&input.attrs);

    TokenStream::from(match input.data {
        Data::Struct(ref struct_data) => {
            deserialize::binde_struct_impl(crate_root, ident, &input, struct_data)
        }
        Data::Enum(ref _enum_data) => deserialize::binde_enum_impl(crate_root, ident, &input),
        _ => unimplemented!(),
    })
}

#[proc_macro_derive(ByteSized, attributes(crate_root, depends))]
pub fn byte_sized_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let crate_root = tools::get_crate_root(&input.attrs);

    TokenStream::from(match input.data {
        Data::Struct(ref struct_data) => {
            byte_sized::byte_sized_struct_impl(crate_root, ident, &input, struct_data)
        }
        Data::Enum(ref _enum_data) => byte_sized::byte_sized_enum_impl(crate_root, ident, &input),
        _ => unimplemented!(),
    })
}

#[proc_macro_derive(Message, attributes(message))]
pub fn message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attr = input.attrs.iter().find(|a| a.path().is_ident("message"));
    if attr.is_none() {
        return TokenStream::from(quote! {compile_error!("no message attribute found on struct");});
    }
    let attr = attr.unwrap();
    let message_type = attr.parse_args::<proc_macro2::TokenStream>().unwrap();
    TokenStream::from(quote! {
        impl #impl_generics crate::messages::MessageArgs for #ident #ty_generics #where_clause {
            const MESSAGE_TYPE: crate::messages::MessageType = #message_type;
        }
    })
}
