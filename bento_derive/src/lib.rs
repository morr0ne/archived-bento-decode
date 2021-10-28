use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit};

#[proc_macro_derive(FromBencode)]
pub fn from_bencode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let mut f = Vec::new();
            let mut names = Vec::new();
            let mut a = Vec::new();

            for field in &fields.named {
                let name = field.ident.as_ref().unwrap();
                f.push(quote! {let mut #name = None;});
                a.push(quote! {
                    b"" => length = Some(u64::decode(value)?),
                });
                names.push(name);
            }

            quote! {
                impl bento::FromBencode for #name {
                    fn decode(object: bento::decode::Object) -> Result<Self, bento::decode::DecodingError>
                    where
                        Self: Sized,
                    {
                        #(#f)*

                        let mut dictionary_decoder = object.try_dictionary()?;
                        while let Some((key, value)) = dict_dec.next_pair()? {
                            match key {
                                #(#a)*
                                unknown_field => {
                                    return Err(bento::decode::DecodingError::UnexpectedField {
                                        field: String::from_utf8_lossy(unknown_field).to_string(),
                                    });
                                }
                            }
                        }

                        Ok(Self{
                            #(#names),*
                        })
                    }
                }
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
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
