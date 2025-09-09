use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Getters)]
/// adds getters to every field of the struct
pub fn getters(input: TokenStream) -> TokenStream {
    
    let input = parse_macro_input!(input as DeriveInput);
    let functions =
    match input.data {
        Data::Struct(input_struct) => {
            input_struct.fields.iter()
                .map(|field| {
                    
                    let field_ident = field.ident.clone().expect("Field for struct must have identifier");
                    let field_type = field.ty.clone();
                    
                    quote! {
                        pub fn #field_ident(&self) -> #field_type {
                            todo!()
                        }
                    }
                }).collect::<Vec<_>>()
        }
        Data::Enum(_) => {
            unimplemented!()
        }
        Data::Union(_) => {
            unimplemented!()
        }
    };
    
    todo!()
}

#[proc_macro_derive(Setters)]
/// adds setters to every field of the struct
pub fn setters(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(SpecificGetters)]
/// adds getters to specific fields that are marked with an attribute macro on those fields
pub fn specific_getters(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(SpecificSetters)]
/// adds setters to specific fields that are marked with an attribute macro on those fields
pub fn specific_setters(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(AllArgsConstructor)]
/// adds a `new` function that has an input of every field for the struct
pub fn all_args_constructor(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(EnumValueList)]
/// adds a function that iterates over the values of the given enum
pub fn enum_value_list(input: TokenStream) -> TokenStream {
    todo!()
}

