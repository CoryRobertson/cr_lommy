#[cfg(test)]
mod tests {
    use cr_lommy_macro::EnumVariantList;

    #[test]
    fn enum_value_list_test() {
        #[derive(EnumVariantList, Eq, PartialEq, Debug)]
        enum CoolEnum {
            First,
            Second,
            Third,
        }

        assert_eq!(CoolEnum::variants().len(), 3);
        assert_eq!(
            CoolEnum::variants(),
            [CoolEnum::First, CoolEnum::Second, CoolEnum::Third]
        );
    }

    #[test]
    fn enum_value_list_with_skip_test() {
        #[derive(EnumVariantList, Eq, PartialEq, Debug)]
        enum CoolEnumWithSkip {
            First,
            #[enum_var_lommy_skip]
            #[allow(unused)]
            Second,
            Third,
        }

        assert_eq!(CoolEnumWithSkip::variants().len(), 2);
        assert_eq!(
            CoolEnumWithSkip::variants(),
            [CoolEnumWithSkip::First, CoolEnumWithSkip::Third]
        );
    }
}
