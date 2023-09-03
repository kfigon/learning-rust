#[cfg(test)]
mod test{
    use std::{fs::read_to_string, num::ParseIntError};


    #[derive(Debug, PartialEq)]
    enum MyError{
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

    fn do_thing() -> Result<(), MyError>{
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

}