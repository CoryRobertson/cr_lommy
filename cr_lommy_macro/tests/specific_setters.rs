#[cfg(test)]
mod tests {
    use cr_lommy_macro::SpecificSetters;

    #[test]
    fn specific_setters_test_derive() {
        #[derive(SpecificSetters, Default)]
        struct TestStruct {
            #[setter]
            a: u32,
            #[allow(unused)]
            b: u32,
            #[setter]
            c: i32,
        }

        let mut s = TestStruct::default();

        s.set_a(5);
        s.set_c(-7);

        assert_eq!(s.a, 5);
        assert_eq!(s.c, -7);

        let mut swapped_value_a = 27;
        let mut swapped_value_c = 15;

        s.swap_a(&mut swapped_value_a);
        s.swap_c(&mut swapped_value_c);

        assert_eq!(swapped_value_a, 5);
        assert_eq!(swapped_value_c, -7);
        assert_eq!(s.a, 27);
        assert_eq!(s.c, 15);
    }
}
