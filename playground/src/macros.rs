#[cfg(test)]
mod test {
    use std::collections::{HashSet, HashMap};

    // declarative macros - code generation. This mod is all declarative
    // procedural macros - more advanced things. Not covered here

    // we can include files at compile time, do conditionals based on env vars, expand expressions and patterns

    // expr, stmt, pat, tt (any token), ty (type), lifetime, literal, ident, path (code path), item (anything), block

    // trace_macros!(true); for debugging

    // hygiene - there're separate macro scopes and implicit variable renaming, so in case of temp vars in macros, there wont be conflicts

    // export macros - #[macro_use] mod module_name;
    // and mark macro rules with #[macro_export]

    macro_rules! foobar {
        ($value:expr, $pattern:pat $(if $guard:expr)?) => {
            match $value {
                $pattern $(if $guard)?=> true,
                _ => false,
            }
        };
    }

    #[test]
    fn my_own_match_test() {
        assert!(!foobar!(123, 4));
        assert!(foobar!(123, x if x % 2 == 1));
    }

    macro_rules! hash_set {
        ($($values:expr),*) => {
            HashSet::from([$($values),*])
        };
        ($($values:expr),+) => { // trailing comma
            hash_set!($ ($values),*)
        };
    }

    #[test]
    fn set_test() {
        let mut exp = HashSet::new();
        exp.insert(4);
        exp.insert(1);
        exp.insert(2);

        let v = hash_set!(1,2,4,1);

        assert_eq!(exp, v);
    }

    macro_rules! make_struct {
        ($name:ident, $min:ty, $max:ty) => {
            #[derive(Debug, PartialEq)]
            struct $name {
                min: $min,
                max: $max,
            }
        };
    }

    #[test]
    fn make_struct_test() {
        make_struct!(TheName, u8, u8);

        let s = TheName {
            min: 3,
            max: 5
        };

        let other = TheName {
            min: 3,
            max: 5
        };

        assert_eq!(s, other);
    }

    #[derive(Debug, PartialEq)]
    enum Json {
        Null,
        Bool(bool),
        Int(i32),
        Str(String),
        Obj(HashMap<String, Json>)
    }
    // these impls can be done with macro
    macro_rules! conversion_to_json {
        ($type:ty, $json_type:tt) => {
            impl From<$type> for Json {
                fn from(value: $type) -> Self {
                    Json::$json_type(value)
                }
            }        
        };
    }
    conversion_to_json!(bool, Bool);
    conversion_to_json!(i32, Int);
    conversion_to_json!(String, Str);

    impl<'a> From<&'a str> for Json {
        fn from(value: &'a str) -> Self {
            Json::Str(value.to_string())
        }
    }

    macro_rules! json {
        ( null ) => { Json::Null };
        ( { $( $key:tt : $val:tt ),* } ) => { 
            Json::Obj(
                HashMap::from(
                    [ $( ($key.to_string(), json!($val)) ),* ]
                )
            )
        };
        ( $other: tt ) => { Json::from($other) };
    }

    #[test]
    fn json_dsl_test() {
        let exp = Json::Obj(
            HashMap::from([
                ("oops".to_string(), Json::Null),
                ("xxx".to_string(), Json::Str("a string".to_string())),
                ("foo".to_string(), Json::Bool(true)),
                ("bar".to_string(), Json::Int(123)),
                ("asd".to_string(), Json::Obj(
                    HashMap::from([
                        ("x".to_string(), Json::Str("the value".to_string()))
                    ])
                )),
            ])
        );

        let got = json!({
            "oops": null,
            "xxx": "a string",
            "foo": true,
            "bar": 123,
            "asd": {
                "x": "the value"
            }
        });

        assert_eq!(exp, got);
    }
}

