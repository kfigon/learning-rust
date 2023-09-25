use std::collections::HashMap;

use nom::{IResult, bytes::complete::{tag, take_until}, branch::alt, character::complete::digit1, sequence::delimited, character::complete::char, multi::separated_list0};

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

impl From<String> for ErrorMsg {
    fn from(value: String) -> Self {
        ErrorMsg(value)
    }
}

impl From<&str> for ErrorMsg {
    fn from(value: &str) -> Self {
        ErrorMsg(value.to_string())
    }
}


fn null(v: &str) -> IResult<&str, Json> {
    let r = tag("null")(v)?;
    Ok((r.0, Json::Null))
}

fn bool(v: &str) -> IResult<&str, Json> {
    let r = alt((
        tag("true"),
        tag("false")
    ))(v)?;

    let parsed = match r.1 {
        "true" => Json::Bool(true),
        "false" => Json::Bool(false),
        _ => unreachable!()
    };
    Ok((r.0, parsed))
}

fn num(v: &str) -> IResult<&str, Json> {
    let r = digit1(v)?;
    let num_val = Json::Num(r.1.parse::<i32>().unwrap());
    Ok((r.0, num_val))
}

fn any_str(v: &str) -> IResult<&str, Json> {
    let r = delimited(
        char('"'),
        take_until("\""),
        char('"'),
    )(v)?;

    let str_val = Json::Str(r.1.to_string());
    Ok((r.0, str_val))
}

fn arr(v: &str) -> IResult<&str, Json> {
    let r = delimited(
        char('['),
        separated_list0(char(','), parser_raw),
        char(']'),
    )(v)?;

    Ok((r.0, Json::Arr(r.1)))
}

fn parser_raw(v: &str) -> IResult<&str, Json> {
    alt((
        null,
        bool,
        num,
        any_str,
        arr
    ))(v)
}

fn parse_json(input: &str) -> Result<Json, ErrorMsg> {
    let r = parser_raw(input).map_err(|e| ErrorMsg(e.to_string()))?;

    Ok(r.1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn null() {
        assert_eq!(parse_json("null"), Ok(Json::Null));
    }
    
    #[test]
    fn integer() {
        assert_eq!(parse_json("15"), Ok(Json::Num(15)));
    }

    #[test]
    fn str() {
        assert_eq!(parse_json("\"foobar\""), Ok(Json::Str("foobar".to_string())));
    }

    #[test]
    fn bool() {
        assert_eq!(parse_json("true"), Ok(Json::Bool(true)));
        assert_eq!(parse_json("false"), Ok(Json::Bool(false)));
    }

    #[test]
    fn arr() {
        assert_eq!(parse_json(r#"[1,2,true,null,"hello"]"#), Ok(Json::Arr(vec![
            Json::Num(1),
            Json::Num(2),
            Json::Bool(true),
            Json::Null,
            Json::Str("hello".to_string()),
        ])));
    }

    #[test]
    fn obj() {
        todo!()
    }

}