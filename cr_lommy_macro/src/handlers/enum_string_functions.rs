use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn enum_to_and_from_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let functions = match input.data {
        Data::Struct(_) => {
            unimplemented!()
        }
        Data::Enum(enum_input) => {

            let enum_ident = input.ident;
            let to_string_function_ident = format_ident!("as_str");

            let idents = enum_input.variants.iter()
                .map(|v| v.ident.clone())
                .collect::<Vec<_>>();
            let ident_strings = enum_input.variants.iter()
                .map(|v| v.ident.to_string())
                .collect::<Vec<_>>();

            // TODO: maybe add a skip attribute which skips specific variants

            quote! {
                impl #enum_ident {
                    pub fn #to_string_function_ident(&self) -> &'static str {
                        match self {
                            #(#enum_ident::#idents => {
                                #ident_strings
                            })*
                        }
                    }

                    pub fn from_str(x: &str) -> Option<#enum_ident> {
                        match x {
                            #(#ident_strings => Some(#enum_ident::#idents)),*,
                            _ => { None }
                        }
                    }
                }
            }
        }
        Data::Union(_) => {
            unimplemented!()
        }
    };

    functions.into()
}