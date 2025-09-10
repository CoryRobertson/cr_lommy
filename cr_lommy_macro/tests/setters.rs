#[cfg(test)]
mod tests {
    use cr_lommy_macro::Setters;

    #[test]
    fn setters_test_derive() {
        #[derive(Setters, Default)]
        struct TestStruct {
            a: u32,
            b: u32,
        }

        let mut s = TestStruct::default();

        s.set_a(5);
        s.set_b(7);

        assert_eq!(s.a, 5);
        assert_eq!(s.b, 7);

        let mut swapped_value_a = 27;
        let mut swapped_value_b = 15;

        s.swap_a(&mut swapped_value_a);
        s.swap_b(&mut swapped_value_b);

        assert_eq!(swapped_value_a, 5);
        assert_eq!(swapped_value_b, 7);
        assert_eq!(s.a, 27);
        assert_eq!(s.b, 15);
    }

    #[test]
    fn skip_some_setters() {
        #[derive(Setters, Default)]
        struct TestStruct {
            a: u32,
            #[setters_lommy_skip]
            #[allow(unused)]
            b: u32,
            #[allow(unused)]
            c: i32,
        }

        let mut s = TestStruct::default();

        s.set_a(5);

        assert_eq!(s.a, 5);

        let mut swapped_value_a = 27;

        s.swap_a(&mut swapped_value_a);

        assert_eq!(swapped_value_a, 5);
        assert_eq!(s.a, 27);
    }
}
