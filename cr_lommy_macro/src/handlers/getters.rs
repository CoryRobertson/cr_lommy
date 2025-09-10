use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{Data, parse_macro_input};

pub(crate) fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let functions = match input.data {
        Data::Struct(input_struct) => {
            let struct_name = input.ident;

            input_struct
                .fields
                .iter()
                .filter_map(|field| {
                    let field_ident = field
                        .ident
                        .clone()
                        .expect("Field for struct must have identifier");
                    let field_type = field.ty.clone();

                    if field
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("getters_lommy_skip"))
                        .is_none()
                    {
                        Some(quote! {
                            impl #struct_name {
                                pub fn #field_ident(&self) -> &#field_type {
                                    &self.#field_ident
                                }
                            }
                        })
                    } else {
                        None
                    }
                })
                .reduce(|mut acc, e| {
                    acc.extend(e);
                    acc
                })
        }
        Data::Enum(_) => {
            unimplemented!()
        }
        Data::Union(_) => {
            unimplemented!()
        }
    };

    functions.unwrap().into()
}
