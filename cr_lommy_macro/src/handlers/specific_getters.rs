use crate::handlers::is_ident_present_in_attr;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{Data, parse_macro_input};

pub(crate) fn specific_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let functions = match input.data {
        Data::Struct(input_struct) => {
            let struct_ident = input.ident;

            input_struct
                .fields
                .into_iter()
                .filter_map(|field| {
                    let getter_attr_present =
                        is_ident_present_in_attr(field.attrs.as_slice(), "getter");

                    if getter_attr_present {
                        let field_ident =
                            field.ident.expect("Field for struct must have identifier");
                        let field_type = field.ty;
                        Some(quote! {
                            impl #struct_ident {
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
