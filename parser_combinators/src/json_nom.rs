use std::collections::HashMap;

use nom::{IResult, bytes::complete::{tag, take_until}, branch::alt, character::complete::{digit1, multispace0}, sequence::{delimited, tuple}, character::complete::{char, alpha1}, multi::{separated_list0, many0}, error::ParseError, combinator::{opt, value}};

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

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

fn null(v: &str) -> IResult<&str, Json> {
    let r = tag("null")(v)?;
    Ok((r.0, Json::Null))
}

fn bool(v: &str) -> IResult<&str, Json> {
    let r = alt((
        value(true, tag("true")),
        value(false, tag("false")),
    ))(v)?;

    Ok((r.0, Json::Bool(r.1)))
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

fn single_obj_key(v: &str) -> IResult<&str, (&str, Json)> {
    let (rest, res) = tuple((
        ws(alpha1), 
        ws(char(':')), 
        ws(parser_raw),
        opt(ws(char(',')))
    ))(v)?;

    Ok((rest, (res.0, res.2)))
}

fn obj(v: &str) -> IResult<&str, Json> {
    let r = delimited(
        ws(char('{')),
        many0(single_obj_key),
        ws(char('}')),
    )(v)?;

    Ok((r.0, Json::Obj(
        HashMap::from_iter(r.1.into_iter()
            .map(|pair| (pair.0.to_string(), pair.1))
        )
    )))
}

fn parser_raw(v: &str) -> IResult<&str, Json> {
    alt((
        ws(null),
        ws(bool),
        ws(num),
        ws(any_str),
        ws(arr),
        ws(obj),
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
    fn arr_with_spaces() {
        assert_eq!(parse_json(r#"[ 1, 2, true, null, "hello" ]"#), Ok(Json::Arr(vec![
            Json::Num(1),
            Json::Num(2),
            Json::Bool(true),
            Json::Null,
            Json::Str("hello".to_string()),
        ])));
    }

    #[test]
    fn obj_simple() {
        let input = r#"{
            foo: "bar",
            asd: 123,
            bar: null,
            sad: true
        }"#;
        assert_eq!(parse_json(input), Ok(Json::Obj(
            HashMap::from_iter(vec![
                ("foo".to_string(), Json::Str("bar".to_string())),
                ("asd".to_string(), Json::Num(123)),
                ("sad".to_string(), Json::Bool(true)),
                ("bar".to_string(), Json::Null)
            ])
        )));
    }

    #[test]
    fn obj_simple_with_spaces() {
        let input = r#"{
            foo  : "bar"  ,
            asd:123,
        }"#;
        assert_eq!(parse_json(input), Ok(Json::Obj(
            HashMap::from_iter(vec![
                ("foo".to_string(), Json::Str("bar".to_string())),
                ("asd".to_string(), Json::Num(123)),
            ])
        )));
    }

    #[test]
    fn obj_nested() {
        let input = r#"{
            foo: {
                ok: false
            }
        }"#;
        assert_eq!(parse_json(input), Ok(Json::Obj(
            HashMap::from_iter(vec![
                ("foo".to_string(), Json::Obj(
                    HashMap::from_iter(vec![
                        ("ok".to_string(), Json::Bool(false))
                    ])
                )),
            ])
        )));
    }

    #[test]
    fn obj_nested_arr() {
        let input = r#"{
            theobj: {
                foo: [ 1,2, 3],
                ok: false
            }
        }"#;
        assert_eq!(parse_json(input), Ok(Json::Obj(
            HashMap::from_iter(vec![
                ("theobj".to_string(), Json::Obj(
                    HashMap::from_iter(vec![
                        ("foo".to_string(), Json::Arr(vec![Json::Num(1),Json::Num(2),Json::Num(3)])),
                        ("ok".to_string(), Json::Bool(false))
                    ])
                )),
            ])
        )));
    }

    #[test]
    fn obj_complicated() {
        let input = r#"{
            foo: "bar",
            asd: 123,
            sad: true,
            theobj: {
                foo: [ 1,2, 3],
                ok: false
            }
        }"#;
        let res = parse_json(input);

        assert_eq!(res, Ok(Json::Obj(
            HashMap::from_iter(vec![
                ("foo".to_string(), Json::Str("bar".to_string())),
                ("asd".to_string(), Json::Num(123)),
                ("sad".to_string(), Json::Bool(true)),
                ("theobj".to_string(), Json::Obj(
                    HashMap::from_iter(vec![
                        ("foo".to_string(), Json::Arr(vec![Json::Num(1),Json::Num(2),Json::Num(3)])),
                        ("ok".to_string(), Json::Bool(false))
                    ])
                )),
            ])
        )));
    }

}