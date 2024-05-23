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

        assert_eq!(
            refs.iter().map(|v| v.do_it()).collect::<Vec<String>>(),
            vec![
                "foo".to_string(),
                "bar".to_string(),
                "asd".to_string(),
                "4".to_string()
            ]
        );
    }

    #[test]
    fn trait_collection_test2() {
        let boxed: Vec<Box<dyn Doer>> = vec![
            Box::new(Data("foo".to_string())),
            Box::new(Data("bar".to_string())),
            Box::new(Data("asd".to_string())),
            Box::new(Data2(4)),
        ];

        assert_eq!(
            boxed.iter().map(|v| v.do_it()).collect::<Vec<String>>(),
            vec![
                "foo".to_string(),
                "bar".to_string(),
                "asd".to_string(),
                "4".to_string()
            ]
        );
    }
}

mod trait_bounds_tests {
    use std::{collections::HashSet, hash::Hash};

    struct HashWrapper<T>(HashSet<T>);

    impl<T> HashWrapper<T>
    where
        T: PartialEq,
        T: Hash,
        T: Eq,
    {
        fn new(s: impl Iterator<Item = T>) -> HashWrapper<T> {
            HashWrapper(HashSet::from_iter(s))
        }

        fn put(&mut self, v: T) {
            self.0.insert(v);
        }
    }

    #[test]
    fn put() {
        let mut h = HashWrapper::new(vec!["foo", "bar", "sad"].into_iter());
        assert_eq!(h.0.len(), 3);
    }

    #[derive(PartialEq, Eq, Debug)]
    struct Entry<T> {
        actual: usize,
        unique: usize,
        first: Option<T>,
    }

    impl<T> Entry<T>
    where
        T: PartialEq + Hash + Eq,
    {
        fn new(v: impl Iterator<Item = T> + Clone) -> Entry<T> {
            let unique = HashSet::<T>::from_iter(v.clone()).len();
            let actual = v.clone().count();
            let first = v.clone().next();

            Entry {
                actual,
                unique,
                first,
            }
        }
    }

    #[test]
    fn entry_str() {
        let e = Entry::new("foo".chars());
        assert_eq!(
            e,
            Entry {
                actual: 3,
                unique: 2,
                first: Some('f')
            }
        );
    }

    #[test]
    fn entry_strings() {
        let e = Entry::new(vec!["foo", "bar", "foo"].into_iter());
        assert_eq!(
            e,
            Entry {
                actual: 3,
                unique: 2,
                first: Some("foo")
            }
        );
    }

    #[test]
    fn entry_ints() {
        let e = Entry::new(vec![123, 432, 1, 2, 2].into_iter());
        assert_eq!(
            e,
            Entry {
                actual: 5,
                unique: 4,
                first: Some(123)
            }
        );
    }
}
