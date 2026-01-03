use crate::handlers::is_ident_present_in_attr;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{Data, parse_macro_input};

pub(crate) fn setters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let functions = match input.data {
        Data::Struct(input_struct) => {
            let struct_name = input.ident;

            input_struct.fields.iter()
                    .filter_map(|field| {

                        let field_ident = field.ident.clone().expect("Field for struct must have identifier");
                        let field_type = field.ty.clone();
                        let swap_value_function_name = quote::format_ident!("swap_{}", field_ident);
                        let set_value_function_name = quote::format_ident!("set_{}", field_ident);

                        let skipped = is_ident_present_in_attr(field.attrs.as_slice(),"setters_lommy_skip");

                        if !skipped {
                            Some(quote! {
                                impl #struct_name {
                                    pub fn #swap_value_function_name(&mut self, new_value: &mut #field_type) {
                                        std::mem::swap(&mut self.#field_ident, new_value);
                                    }
                                    pub fn #set_value_function_name(&mut self, new_value: #field_type) {
                                        self.#field_ident = new_value;
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
