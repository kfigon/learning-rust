#[cfg(test)]
mod tests {
    use std::ops::Deref;

    struct WrapArray {
        data: Vec<i32>,
        idx: i32,
    }

    // to automatically unwrap values from the wrapping type
    impl Deref for WrapArray {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.data[self.idx as usize]
        }
    }

    #[test]
    fn deref_test() {
        let d = WrapArray{
            data: vec![4,5,6],
            idx: 2,
        };

        assert_eq!(*d, 6);
    }

    fn my_foo(v: &impl AsRef<str>) -> &str {
        v.as_ref()
    }

    #[test]
    fn asref_test() {
        assert_eq!(my_foo(&"foo"), "foo");
        assert_eq!(my_foo(&"foo".to_string()), "foo");
    }

    #[test]
    fn borrow_test() {
        // todo!()
        // this is similar to asref, but used in context of hashmaps
    }
}