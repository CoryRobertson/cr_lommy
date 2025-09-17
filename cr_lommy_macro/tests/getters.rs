#[cfg(test)]
mod tests {
    use cr_lommy_macro::Getters;

    #[test]
    fn getters_test_derive() {
        #[derive(Getters, Default)]
        struct TestStruct {
            #[getters_lommy_mut]
            a: u32,
            b: u32,
        }

        let mut s = TestStruct::default();

        assert_eq!(*s.a(), 0);
        assert_eq!(*s.b(), 0);


        s.a = 5;
        s.b = 7;

        assert_eq!(*s.a(), 5);
        assert_eq!(*s.b(), 7);

        let a_mut = s.a_mut();
        *a_mut = 31;

        assert_eq!(*s.a(),31);

    }

    #[test]
    fn skip_some_getters() {
        #[derive(Getters, Default)]
        struct TestStruct {
            a: u32,
            #[getters_lommy_skip]
            b: u32,
            #[allow(unused)]
            c: i32,
        }

        let mut s = TestStruct::default();

        assert_eq!(*s.a(), 0);

        s.a = 5;
        s.b = 7;

        assert_eq!(*s.a(), 5);
    }
}
