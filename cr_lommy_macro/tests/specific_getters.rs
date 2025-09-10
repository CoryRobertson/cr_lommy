#[cfg(test)]
mod tests {
    use cr_lommy_macro::SpecificGetters;

    #[test]
    fn specific_getters_test_derive() {
        #[derive(SpecificGetters, Default)]
        struct TestStruct {
            #[getter]
            a: u32,
            #[allow(unused)]
            b: u32,
            #[getter]
            c: i32,
        }

        let mut s = TestStruct::default();

        assert_eq!(*s.a(), 0);
        assert_eq!(*s.c(), 0);

        s.a = 5;
        assert_eq!(*s.a(), 5);

        s.c = -14;
        assert_eq!(*s.c(), -14);
    }
}
