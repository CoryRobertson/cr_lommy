use cr_lommy_macro::{EnumString, EnumVariantList};

#[test]
fn test_to_string() {

    #[derive(EnumString, Debug)]
    enum CoolEnum {
        First,
        Second,
        Third
    }

    assert_eq!("First", CoolEnum::First.as_str());
    assert_eq!("Second", CoolEnum::Second.as_str());
    assert_eq!("Third", CoolEnum::Third.as_str());

}

#[test]
fn test_from_string() {
    #[derive(EnumString, EnumVariantList, Eq, PartialEq, Debug)]
    #[allow(non_camel_case_types,)]
    enum TestEnum {
        First,
        Second,
        Third,
        WeiRdCapitaLLLs,
        /// Comments!!!
        Nu1mbe3rz0,
        snake_case,
        #[allow(unused)]
        SCREAMING_SNAKE_CASE,

    }

    for e in TestEnum::variants() {
        let to_string = e.as_str();
        let from_string = TestEnum::from_str(to_string);
        assert_eq!(e,from_string.unwrap());
    }

    assert_eq!(TestEnum::from_str("n o t a v a l i d e n u m n a m e !@#!#$T#%^45675674-="), None);

}