use std::{collections::HashMap, str::Chars};

#[test]
fn encode_int_test() {
    assert_eq!("i45e", encode_int(45));
}

#[test]
fn decode_int_test() {
    assert_eq!(Ok(BencodeObj::Int(45)), decode_generic_str("i45e"));
    assert_eq!(Ok(BencodeObj::Int(-123)), decode_generic_str("i-123e"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_generic_str("123"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_generic_str("i"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_generic_str("i1"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_generic_str("ie"));
    assert_eq!(Err(ErrorMsg("missing len")), decode_generic_str("12e"));
    assert_eq!(Err(ErrorMsg("cant parse")), decode_generic_str("iasdfe"));
}

#[test]
fn encode_str_test() {
    assert_eq!("3:foo", encode_str("foo"));
    assert_eq!("4:1234", encode_str("1234"));
    assert_eq!("5:4:foo", encode_str("4:foo"));
}

#[test]
fn decode_dict_test() {
    let hash: HashMap<String, BencodeObj> = HashMap::from([
        ("foo".to_owned(), BencodeObj::Int(2)),
        ("bar".to_owned(), BencodeObj::Str("oopsie".to_owned())),
    ]);

    assert_eq!(Ok(BencodeObj::Dict(hash)), decode_generic_str("d3:fooi2e3:bar6:oopsiee"));
}

#[test]
fn decode_str_test() {
    assert_eq!(Ok(BencodeObj::Str("foo".to_owned())), decode_generic_str("3:foo"));
    assert_eq!(Ok(BencodeObj::Str("asdfgqwe123r".to_owned())), decode_generic_str("12:asdfgqwe123r"));
    assert_eq!(Ok(BencodeObj::Str("1234".to_owned())), decode_generic_str("4:1234"));
    assert_eq!(Ok(BencodeObj::Str("4:foo".to_owned())), decode_generic_str("5:4:foo"));
    assert_eq!(Ok(BencodeObj::Str("4:fo".to_owned())), decode_generic_str("4:4:foo"));
    assert_eq!(Err(ErrorMsg("invalid str")), decode_generic_str("0:asd"));
    assert_eq!(Err(ErrorMsg("invalid str")), decode_generic_str("-1:"));
    assert_eq!(Err(ErrorMsg("invalid str")), decode_generic_str("0"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_generic_str("6:4:foo"));
    assert_eq!(Err(ErrorMsg("invalid str")), decode_generic_str("asd"));
    assert_eq!(Err(ErrorMsg("missing len")), decode_generic_str("4asd"));
}

#[derive(Debug, PartialEq, Eq)]
struct ErrorMsg(&'static str);

fn encode_int(i: i32) -> String {
    format!("i{i}e")
}

fn decode_int(i: &str) -> Result<i32, ErrorMsg> {
    if i.len() < 3 {
        return Err(ErrorMsg("invalid len"));
    }
    let mut chars = i.chars();
    match chars.next() {
        Some('i') => (),
        _ => return Err(ErrorMsg("missing end or beginning tags")),
    }

    let mut num = String::new();

    while let Some(c) = chars.next() {
        if c == 'e' {
            break;
        }
        num.push(c)
    }

    let v = num.parse::<i32>().map_err(|_| ErrorMsg("cant parse"))?;

    Ok(v)
}

fn encode_str(s: &str) -> String {
    format!("{}:{}", s.len(), s)
}

fn decode_str(s: &str) -> Result<String, ErrorMsg> {
    let mut num = String::new();
    let mut chars = s.chars().into_iter();
    while let Some(c) = chars.next() {
        match c {
            '1'..='9' => num.push(c),
            ':' => break,
            _ => return Err(ErrorMsg("missing len")),
        }
    }
    let len = num.parse::<usize>().map_err(|_| ErrorMsg("invalid len"))?;

    let mut out = String::new();
    for _ in 0..len {
        match chars.next() {
            Some(c) => out.push(c),
            _ => return Err(ErrorMsg("invalid len")),
        }
    }

    Ok(out)
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum BencodeObj {
    Str(String), // 3:foo
    Int(i32), //i1234e
    List(Vec<BencodeObj>), //l i1e i2e 3:str e // without spaces
    Dict(HashMap<String, BencodeObj>), //d 3:foo i1e 3:str 3:foo e // without spaces
}

fn decode_generic_str(s: &str) -> Result<BencodeObj, ErrorMsg> {
    match s.chars().next() {
        Some('1'..='9') => decode_str(s).map(|v| BencodeObj::Str(v)),
        Some('i') => decode_int(s).map(|v| BencodeObj::Int(v)),
        Some('d') => decode_dict(s).map(|v| BencodeObj::Dict(v)),
        _ => Err(ErrorMsg("invalid str")),
    }
}

fn decode_dict(s: &str) -> Result<HashMap<String, BencodeObj>, ErrorMsg> {
    let mut chars = s.chars();
    match (chars.next(), chars.next_back()) {
        (Some('d'), Some('e')) => (),
        _ => return Err(ErrorMsg("invalid dict")),
    }

    let mut out: HashMap<String, BencodeObj> = HashMap::new();

    loop {
        let key = decode_str(chars.as_str())?;
        advance_iter(&mut chars, &BencodeObj::Str(key.clone()));
        let val = decode_generic_str(chars.as_str())?;
        advance_iter(&mut chars, &val);

        out.insert(key, val);
        if chars.as_str().len() == 0 {
            break;
        }
    }

    Ok(out)
}

fn advance_iter(chars: &mut Chars, obj: &BencodeObj) {
    match obj {
        BencodeObj::Str(s) => {
            let f = s.len() as f64 + 1.0;
            let len = f.log10().ceil() as usize;
            for _ in 0..len {
                chars.next();
            }
            chars.next();
            for _ in 0..s.len() {
                chars.next();
            }
        },
        BencodeObj::Int(i) => {
            chars.next();
            let f = *i as f64 + 1.0;
            let len = f.log10().ceil() as usize;
            for _ in 0..len {
                chars.next();
            }
            chars.next();
        }
        BencodeObj::List(_) => todo!(),
        BencodeObj::Dict(_) => todo!(),
    }
}