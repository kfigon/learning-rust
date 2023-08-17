#[cfg(test)]
mod tests {

    trait Doer {
        fn do_it(&self) -> String;
    }

    struct Data(String);
    struct Data2(i32);

    impl Doer for Data {
        fn do_it(&self) -> String {
            self.0.to_owned()
        }
    }

    impl Doer for Data2 {
        fn do_it(&self) -> String {
            self.0.to_string()
        }
    }

    // &impl Doer is also ok
    fn static_dispatch(v: impl Doer) -> String {
        v.do_it()
    }

    fn dynamic_dispatch(v: &dyn Doer) -> String {
        v.do_it()
    }

    fn dynamic_dispatch_2(v: Box<dyn Doer>) -> String {
        v.do_it()
    }

    #[test]
    fn impl_test() {
        let d = Data("foobar".to_string());
        assert_eq!(static_dispatch(d), "foobar".to_string())
    }

    #[test]
    fn dynamic_dispatch_test() {
        let d = Data("foobar".to_string());

        assert_eq!(dynamic_dispatch(&d), "foobar".to_string());
        assert_eq!(dynamic_dispatch_2(Box::new(d)), "foobar".to_string());
    }

    #[test]
    fn trait_collection_test() {
        let x1 = Data("foo".to_string());
        let x2 = Data("bar".to_string());
        let x3 = Data("asd".to_string());
        let x4 = Data2(4);

        let refs: Vec<&dyn Doer> = vec![&x1, &x2, &x3, &x4];

        assert_eq!(refs.iter().map(|v| v.do_it()).collect::<Vec<String>>(), vec!["foo".to_string(), "bar".to_string(), "asd".to_string(), "4".to_string()]);
    }

    #[test]
    fn trait_collection_test2() {
        let boxed: Vec<Box<dyn Doer>> = vec![
            Box::new(Data("foo".to_string())),
            Box::new(Data("bar".to_string())),
            Box::new(Data("asd".to_string())),
            Box::new(Data2(4)),
        ];

        assert_eq!(boxed.iter().map(|v| v.do_it()).collect::<Vec<String>>(), vec!["foo".to_string(), "bar".to_string(), "asd".to_string(), "4".to_string()]);
    }
}