use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, parse_macro_input, Meta, Expr, Lit};
use crate::handlers::is_ident_present_in_attr;

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

            let custom_function_name = input.attrs.iter()
                .find(|attr| attr.path().is_ident("all_args_constructor"))
                .map(|a| {
                    match &a.meta {
                        Meta::NameValue(nv) => {
                            match &nv.value {
                                Expr::Lit(expr_lit) => {
                                    match &expr_lit.lit {
                                        Lit::Str(name) => {
                                            name.value()
                                        }
                                        _ => {
                                            unimplemented!()
                                        }
                                    }
                                }
                                _ => {
                                    unimplemented!()
                                }
                            }
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                })
                .map(|v| format_ident!("{}", v))
                .unwrap_or(format_ident!("new_all_args"));

            quote! {
                impl #struct_name {
                    pub fn #custom_function_name(#(#field_idents: #field_types),*) -> Self {
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
