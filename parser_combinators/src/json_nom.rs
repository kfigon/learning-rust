use std::collections::HashMap;

#[derive(Debug,PartialEq)]
enum Json {
    Null,
    Str(String),
    Num(i32),
    Bool(bool),
    Arr(Vec<Json>),
    Obj(HashMap<String, Json>),
}

#[derive(Debug,PartialEq)]
struct ErrorMsg(String);

impl TryFrom<&str> for Json {
    type Error = ErrorMsg;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn null() {
        assert_eq!("null".try_into(), Ok(Json::Null));
    }

    #[test]
    fn integer() {
        assert_eq!("15".try_into(), Ok(Json::Num(15)));
    }

    #[test]
    fn str() {
        assert_eq!("foobar".try_into(), Ok(Json::Str("foobar".to_string())));
    }

    #[test]
    fn bool() {
        assert_eq!("true".try_into(), Ok(Json::Bool(true)));
        assert_eq!("false".try_into(), Ok(Json::Bool(false)));
    }

    #[test]
    fn arr() {
        todo!()
    }

    #[test]
    fn obj() {
        todo!()
    }

}