#[cfg(test)]
mod test {
    use std::collections::HashMap;

    // immutable is easy, stuff everything in single struct. Mutable is hard
    // my first attempt was to store map in Functions struct. This is invalid from ownership perspective. we need some parent

    type Closure = dyn Fn(&mut Functions) -> String;
    struct Callbacks {
        map: HashMap<String, Box<Closure>>,
    }

    impl Callbacks {
        fn new() -> Self {
            let mut map: HashMap<String, Box<Closure>> = HashMap::new(); // from_iter doesnt work
            map.insert("foobar".to_string(), Box::new(Functions::foobar));
            map.insert("asdf".to_string(), Box::new(Functions::asdf));

            Self { map }
        }
    }

    struct Functions;
    impl Functions {
        fn foobar(&mut self) -> String {
            "Foobar".to_string()
        }

        fn asdf(&mut self) -> String {
            "Asdf".to_string()
        }
    }

    #[test]
    fn map_closures() {
        let mut f = Functions;
        let map = Callbacks::new();
        let func = map.map.get("foobar").unwrap(); // map borrow happens here
        let res = func(&mut f);
        assert_eq!(res, "Foobar");
        assert_eq!(map.map.get("asdf").unwrap()(&mut f), "Asdf"); // again - it's shared ref, so it's ok
    }
}
