#[cfg(test)]
mod tests {
    use cr_lommy_macro::AllArgsConstructor;

    #[test]
    fn all_args_constructor() {
        #[derive(AllArgsConstructor)]
        struct TestStruct {
            a: u32,
            b: f32,
            c: String,
        }

        let new_test_struct = TestStruct::new_all_args(5, 1.2, "Cool test!!!".to_string());

        assert_eq!(new_test_struct.a, 5);
        assert_eq!(new_test_struct.b, 1.2);
        assert_eq!(new_test_struct.c, "Cool test!!!".to_string());
    }

    #[test]
    fn all_args_constructor_custom_name() {
        #[derive(AllArgsConstructor)]
        #[all_args_constructor = "new"]
        struct TestStruct {
            a: u32,
            b: f32,
            c: String,
        }

        let new_test_struct = TestStruct::new(5, 1.2, "Cool test!!!".to_string());

        assert_eq!(new_test_struct.a, 5);
        assert_eq!(new_test_struct.b, 1.2);
        assert_eq!(new_test_struct.c, "Cool test!!!".to_string());
    }
}
