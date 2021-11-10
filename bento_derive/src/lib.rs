use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Bencode)]
pub fn bencode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = quote! {};

    gen.into()
}

#[proc_macro_derive(FromBencode)]
pub fn from_bencode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    if let Data::Struct(data) = input.data {}

    let gen = quote! {
        impl FromBencode for #name {

        }
    };

    gen.into()
}

#[proc_macro_derive(ToBencode)]
pub fn to_bencode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    if let Data::Struct(data) = input.data {}

    let gen = quote! {
        impl ToBencode for #name {

        }
    };

    gen.into()
}
