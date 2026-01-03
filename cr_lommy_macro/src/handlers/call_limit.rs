use proc_macro::TokenStream;
use darling::FromMeta;
use syn::{ItemFn, Stmt};

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct CallLimitArgs {
    limit: Option<usize>,
    assert_non_debug: Option<bool>,
}




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

    let att: CallLimitArgs = syn::parse(attr).unwrap();


    let limit = att.limit.unwrap_or(1);
    let assert_non_debug = att.assert_non_debug.unwrap_or(false);

    let mut item_fn: ItemFn = syn::parse(item.clone()).unwrap();

    let added_code: Stmt = syn::parse_quote! {
        {
            use std::sync::atomic::AtomicUsize;
            static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
            let call_count = CALL_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            if #assert_non_debug {
                assert!(call_count <= #limit, "Call count exceeded, call_count: {}, limit: {}", call_count, #limit);
            } else {
                debug_assert!(call_count <= #limit, "Call count exceeded, call_count: {}, limit: {}", call_count, #limit);
            }
        }
    };

    item_fn.block.stmts.insert(0, added_code);

    quote::quote!(#item_fn).into()
}