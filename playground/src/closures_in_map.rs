#[cfg(test)]
mod test {
    use std::collections::HashMap;

    type Closure = dyn Fn(&Foo) -> String;
    struct Foo {
        map: HashMap<String, Box<Closure>>,
    }

    impl Foo {
        fn new() -> Self {
            let mut map: HashMap<String, Box<Closure>> = HashMap::new();
            map.insert("foobar".to_string(), Box::new(&Foo::foobar));
            map.insert("asdf".to_string(), Box::new(&Foo::asdf));

            Self { map }
        }

        fn foobar(&self) -> String {
            "Foobar".to_string()
        }

        fn asdf(&self) -> String {
            "Asdf".to_string()
        }
    }

    #[test]
    fn map_closures() {
        let f = Foo::new();
        assert_eq!(f.map.get("foobar").unwrap()(&f), "Foobar");
        assert_eq!(f.map.get("asdf").unwrap()(&f), "Asdf");
    }
}
