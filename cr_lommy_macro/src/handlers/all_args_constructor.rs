use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn all_args_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(input_struct) => {
            let struct_name = input.ident;

            let field_idents = input_struct
                .fields
                .iter()
                .map(|field| {
                    let field_ident = field
                        .ident
                        .clone()
                        .expect("Field for struct must have identifier");
                    field_ident
                })
                .collect::<Vec<_>>();
            let field_types = input_struct
                .fields
                .iter()
                .map(|field| {
                    let field_type = field.ty.clone();
                    field_type
                })
                .collect::<Vec<_>>();

            quote! {
                impl #struct_name {
                    pub fn new_all_args(#(#field_idents: #field_types),*) -> Self {
                        Self {
                            #(#field_idents,)*
                        }
                    }
                }
            }
            .into()
        }
        Data::Enum(_) => {
            unimplemented!()
        }
        Data::Union(_) => {
            unimplemented!()
        }
    }
}
