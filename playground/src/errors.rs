#[cfg(test)]
mod test {
    use std::{fmt::Display, fs::read_to_string, num::ParseIntError};

    #[derive(Debug, PartialEq)]
    enum MyError {
        GenericError(String),
        ReadFile(String),
        ParseError(ParseIntError),
    }

    // so the ? operator on read_to_string for MyError
    impl From<std::io::Error> for MyError {
        fn from(value: std::io::Error) -> Self {
            MyError::ReadFile(value.to_string())
        }
    }

    // so the ? operator on parse for MyError
    impl From<ParseIntError> for MyError {
        fn from(value: ParseIntError) -> Self {
            MyError::ParseError(value)
        }
    }

    fn do_thing() -> Result<(), MyError> {
        let _num = "foobar".parse::<i32>()?;
        if _num == 0 {
            return Err(MyError::GenericError("0 found".to_string()));
        }

        let _data = read_to_string("foo")?;

        Ok(())
    }

    #[test]
    fn error_handling_test() {
        let r = do_thing();
        match r {
            Ok(_) => panic!("expected failure"),
            Err(e) => match e {
                MyError::ParseError(_) => (),
                _ => panic!("epected parse error"),
            },
        }
    }

    #[derive(Debug)]
    struct ErrorA;
    impl std::error::Error for ErrorA {}
    impl Display for ErrorA {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Error A")
        }
    }

    #[derive(Debug)]
    struct ErrorB;
    impl std::error::Error for ErrorB {}
    impl Display for ErrorB {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Error B")
        }
    }

    fn get_dynamic_error(b: bool) -> Result<(), Box<dyn std::error::Error>> {
        if b {
            Err(Box::new(ErrorA))
        } else {
            Err(Box::new(ErrorB))
        }
    }

    #[test]
    fn dynaminc_error() {
        let res = get_dynamic_error(true);
        assert_eq!(res.err().unwrap().to_string(), "Error A")
    }
}
