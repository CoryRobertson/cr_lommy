use crate::handlers::is_ident_present_in_attr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn enum_value_list(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let functions = match input.data {
        Data::Enum(input_enum) => {
            let enum_ident = input.ident;

            let non_skipped_variant_list = input_enum
                .variants
                .into_iter()
                .filter(|v| !is_ident_present_in_attr(v.attrs.as_slice(), "enum_var_lommy_skip"))
                .map(|variant| {
                    if !variant.fields.is_empty() {
                        panic!(
                            "Enum variants with fields are currently not supported: {}",
                            variant.ident
                        );
                    }

                    variant.ident
                })
                .collect::<Vec<_>>();

            let length = non_skipped_variant_list.len();

            quote! {
                impl #enum_ident {
                    pub fn variants() -> [Self ; #length] {
                        [#(Self::#non_skipped_variant_list),*]
                    }
                }
            }
        }
        Data::Struct(_) => {
            unimplemented!()
        }
        Data::Union(_) => {
            unimplemented!()
        }
    };

    functions.into()
}
