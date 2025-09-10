use proptest::proptest;
use cr_lommy_macro::{AllArgsConstructor, Getters, Setters};

proptest! {
    #[test]
    fn test1(a in 0..u32::MAX, b in 0f32..f32::MAX, c in "\\PC*",a2 in 0..u32::MAX, b2 in 0f32..f32::MAX, c2 in "\\PC*") {
            
            #[derive(Getters, Setters, AllArgsConstructor)]
            struct Test1Struct {
                a: u32,
                b: f32,
                c: String,
            }
            
            let mut t1s: Test1Struct = Test1Struct::new_all_args(a,b,c.to_string());
    
            assert_eq!(*t1s.a(),a);
            assert_eq!(*t1s.b(),b);
            assert_eq!(*t1s.c(),c.to_string());
        
            t1s.set_a(a2);
            t1s.set_b(b2);
            t1s.set_c(c2.to_string());
        
            assert_eq!(*t1s.a(),a2);
            assert_eq!(*t1s.b(),b2);
            assert_eq!(*t1s.c(),c2.to_string());
    
        }
}