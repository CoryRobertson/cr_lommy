mod handlers;

use proc_macro::TokenStream;

#[proc_macro_derive(Getters, attributes(getters_lommy_skip, getters_lommy_mut))]
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
/// s.set_b(52); // this line fails to compile because `b` is skipped
/// let mut new_value = 62;
/// s.swap_b(&mut new_value);
///
/// ```
pub fn setters(input: TokenStream) -> TokenStream {
    handlers::setters::setters(input)
}

#[proc_macro_derive(SpecificGetters, attributes(getter))]
/// adds getters to specific fields that are marked with an attribute macro on those fields
/// ```compile_fail, E0599
/// use cr_lommy_macro::SpecificGetters;
/// #[derive(SpecificGetters, Default)]
/// struct TestStruct {
///   #[getter]
///   a: u32,
///   #[allow(unused)]
///   b: u32,
///   #[getter]
///   c: i32,
///  }
///
///  let mut s = TestStruct::default();
///
///  assert_eq!(*s.a(), 0);
///  assert_eq!(*s.b(), 0); // this line fails to compile because we did not add a getter to the field `b`
///  assert_eq!(*s.c(), 0);
///
///
///  s.a = 5;
///  assert_eq!(*s.a(), 5);
///
///  s.c = -14;
///  assert_eq!(*s.c(), -14);
/// ```
pub fn specific_getters(input: TokenStream) -> TokenStream {
    handlers::specific_getters::specific_getters(input)
}

#[proc_macro_derive(SpecificSetters, attributes(setter))]
/// adds setters to specific fields that are marked with an attribute macro on those fields
/// ```compile_fail, E0599
/// use cr_lommy_macro::SpecificSetters;
/// #[derive(SpecificSetters, Default)]
///  struct TestStruct {
///     #[setter]
///     a: u32,
///     #[allow(unused)]
///     b: u32,
///     #[setter]
///     c: i32,
///  }
///
///  let mut s = TestStruct::default();
///  s.set_a(4);
///  s.set_b(5); // the field `b` did not have a setter attribute added to it
///  s.set_c(6);
///
/// ```
pub fn specific_setters(input: TokenStream) -> TokenStream {
    handlers::specific_setters::specific_setters(input)
}

#[proc_macro_derive(AllArgsConstructor)]
/// adds a `new` function that has an input of every field for the struct
/// ```rust
/// use cr_lommy_macro::AllArgsConstructor;
///
/// #[derive(AllArgsConstructor)]
///  struct TestStruct {
///     int_field: u32,
///     float_field: f32,
///     string_field: String,
///  }
///
///  let new_test_struct = TestStruct::new_all_args(5, 1.2, "Cool test!!!".to_string());
///
///  assert_eq!(new_test_struct.int_field, 5);
///  assert_eq!(new_test_struct.float_field, 1.2);
///  assert_eq!(new_test_struct.string_field, "Cool test!!!".to_string());
/// ```
pub fn all_args_constructor(input: TokenStream) -> TokenStream {
    handlers::all_args_constructor::all_args_constructor(input)
}

#[proc_macro_derive(EnumVariantList, attributes(enum_var_lommy_skip))]
/// adds a function that iterates over the values of the given enum
pub fn enum_value_list(input: TokenStream) -> TokenStream {
    handlers::enum_variant_list::enum_value_list(input)
}

#[proc_macro_derive(EnumString)]
/// Adds a to_str and from_str function to a given enum, im not sure if I like these function names yet, also might make this derive implement From<> and Into<> for each
pub fn enum_to_and_from_string(input: TokenStream) -> TokenStream {
    handlers::enum_string_functions::enum_to_and_from_string(input)
}

#[proc_macro_attribute]
/// Adds a block of code to the top of a function that looks like this
/// ```compile_fail, rust
/// {
///      use std::sync::atomic::AtomicUsize;
///      static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
///      let call_count = CALL_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
///      if #assert_non_debug {
///            assert!(call_count <= #limit, "Call count exceeded, call_count: {}, limit: {}", call_count, #limit);
///      } else {
///            debug_assert!(call_count <= #limit, "Call count exceeded, call_count: {}, limit: {}", call_count, #limit);
///      }
///  }
/// ```
/// This code will debug_assert that a function is not called by more than a specified amount
/// You can denote a function like this
/// ```should_panic,rust
/// use cr_lommy_macro::call_limit;
///
/// #[call_limit(limit = 3, assert_non_debug = true)]
/// fn function_with_a_call_limit() {}
///
/// function_with_a_call_limit();
/// function_with_a_call_limit();
/// function_with_a_call_limit();
/// function_with_a_call_limit(); // panic happens here!
///
/// ```
/// If no limit is specified then a limit of 1 is assumed
pub fn call_limit(attr: TokenStream, item: TokenStream) -> TokenStream {
    handlers::call_limit::call_limit(attr, item)
}
