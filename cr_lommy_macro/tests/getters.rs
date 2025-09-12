#[cfg(test)]
mod tests {
    use cr_lommy_macro::{AllArgsConstructor, Getters};

    #[test]
    fn getters_test_derive() {
        #[derive(Getters, Default)]
        struct TestStruct {
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

    #[test]
    fn custom_getter_function() {
        #[derive(AllArgsConstructor, Getters)]
        struct TestStruct {
            a: f32,
            #[getter_fn(|a: &u32| -> u32 { *a + 1 })]
            b: u32,
        }

        let a = TestStruct::new_all_args(7.1, 32);

        let b = a.b();

        assert!(b == 33);
    }
}
