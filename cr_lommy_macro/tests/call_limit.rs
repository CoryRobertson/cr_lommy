use cr_lommy_macro::call_limit;

#[test]
#[should_panic]
fn test_call_once() {
    #[call_limit]
    fn test() {}

    test();

    test();
}

#[test]
fn test_call_once_non_panic() {
    #[call_limit]
    fn test() {}

    test();
}

#[test]
fn test_call_non_once() {
    #[call_limit(limit = 2)]
    fn test() {}

    test();

    test();
}
