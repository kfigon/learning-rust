#[cfg(test)]
mod tests {

    trait Doer {
        fn do_it(&self) -> String;
    }

    struct Data(String);

    impl Doer for Data {
        fn do_it(&self) -> String {
            self.0.to_owned()
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
}