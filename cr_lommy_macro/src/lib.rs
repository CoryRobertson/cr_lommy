mod handlers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(Getters, attributes(getters_lommy_skip))]
/// adds getters to every field of the struct
/// ```compile_fail,E0599
///
/// use cr_lommy_macro::Getters;
///
/// #[derive(Getters, Default)]
/// struct TestStruct {
///   a: u32,
///   #[getters_lommy_skip]
///   b: u32,
/// }
///
/// let s = TestStruct::default();
///
/// assert_eq!(*s.a(), 0);
/// assert_eq!(*s.b(), 0); // field `b` is skipped therefore this will not compile
///
/// ```
pub fn getters_derive(input: TokenStream) -> TokenStream {
    handlers::getters::getters(input)
}

#[proc_macro_derive(Setters, attributes(setters_lommy_skip))]
/// adds setters to every field of the struct
/// ```compile_fail,E0599
///
/// use cr_lommy_macro::Setters;
///
/// #[derive(Setters, Default)]
/// struct TestStruct {
///   a: u32,
///   #[setters_lommy_skip]
///   b: u32,
/// }
///
/// let mut s = TestStruct::default();
///
/// s.set_a(52);
/// s.set_b(52); // set_b is skipped
/// let mut new_value = 62;
/// s.swap_b(&mut new_value);
///
/// ```
pub fn setters(input: TokenStream) -> TokenStream {
    handlers::setters::setters(input)
}

#[proc_macro_derive(SpecificGetters, attributes(getter))]
/// adds getters to specific fields that are marked with an attribute macro on those fields
pub fn specific_getters(input: TokenStream) -> TokenStream {
    handlers::specific_getters::specific_getters(input)
}

#[proc_macro_derive(SpecificSetters, attributes(setter))]
/// adds setters to specific fields that are marked with an attribute macro on those fields
pub fn specific_setters(input: TokenStream) -> TokenStream {
    handlers::specific_setters::specific_setters(input)
}

#[proc_macro_derive(AllArgsConstructor)]
/// adds a `new` function that has an input of every field for the struct
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

#[proc_macro_derive(EnumVariantList, attributes(enum_var_lommy_skip))]
/// adds a function that iterates over the values of the given enum
pub fn enum_value_list(input: TokenStream) -> TokenStream {
    handlers::enum_variant_list::enum_value_list(input)
}
