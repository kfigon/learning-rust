#[cfg(test)]
mod tests {
    #[test]
    fn immutable_test() {
        // Fn
        let to_string = |v: i32| v.to_string();

        assert_eq!(to_string(15), "15");
    }

    #[test]
    fn mutable_test() {
        let mut v = "hello".to_string();
        // FnMut
        let mut inc = || {
            v += "!";
        };

        inc();
        inc();
        assert_eq!(v, "hello!!");

        // inc(); // compilation fail - multiple borrows
    }

    #[test]
    fn move_test() {
        let mut v = "hello".to_string();
        // FnOnce
        let inc = move || -> String {
            v += " world";
            v
        };

        assert_eq!(inc(), "hello world".to_string());        
        // assert_eq!(v, "hello".to_string()); // compilation fail - borrow after move
    }
}