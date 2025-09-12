use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, parse_macro_input};
use syn::{DeriveInput, ExprClosure, ReturnType, parse_quote};

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

                    let has_skip_attribute = field
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("getters_lommy_skip"))
                        .is_some();

                    let has_custom_fn_attribute = field
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("getter_fn"));

                    if !has_skip_attribute {
                        let get_function_quote = match has_custom_fn_attribute {
                            None => {
                                quote! {
                                    impl #struct_name {
                                        pub fn #field_ident(&self) -> &#field_type {
                                            &self.#field_ident
                                        }
                                    }
                                }
                            }
                            Some(attr) => {
                                let closure = attr
                                    .parse_args::<ExprClosure>()
                                    .expect("Failed to parse closure");

                                let inputs = closure.inputs.clone();
                                let outputs = closure.output.clone();
                                let body = closure.body.clone();

                                let output_type = match outputs {
                                    ReturnType::Default => {
                                        panic!("No return type :(")
                                    }
                                    ReturnType::Type(_arrow, b) => b,
                                };

                                let closure_quote: ExprClosure = parse_quote! {
                                    |#inputs| -> #output_type { #body }
                                };

                                quote! {
                                    impl #struct_name {
                                        pub fn #field_ident(&self) -> #output_type {
                                            #closure_quote(&self.#field_ident)

                                        }
                                    }
                                }
                            }
                        };

                        Some(get_function_quote)
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
