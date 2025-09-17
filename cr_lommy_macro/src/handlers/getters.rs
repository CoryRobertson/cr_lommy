use crate::handlers::is_ident_present_in_attr;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
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

                    let skipped =
                        is_ident_present_in_attr(field.attrs.as_slice(), "getters_lommy_skip");

                    if !skipped {
                        let get_mut_attr =
                            is_ident_present_in_attr(field.attrs.as_slice(), "getters_lommy_mut");

                        let mut functions = quote! {
                            impl #struct_name {
                                pub fn #field_ident(&self) -> &#field_type {
                                    &self.#field_ident
                                }
                            }
                        };

                        if get_mut_attr {
                            let get_mut_ident = format_ident!("{}_mut", field_ident);

                            functions.extend(quote! {
                                impl #struct_name {
                                    pub fn #get_mut_ident(&mut self) -> &mut #field_type {
                                        &mut self.#field_ident
                                    }
                                }
                            });
                        }

                        Some(functions)
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
