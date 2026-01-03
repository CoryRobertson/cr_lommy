use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{ItemFn, Stmt};

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct CallLimitArgs {
    limit: Option<usize>,
    assert_non_debug: Option<bool>,
}

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
