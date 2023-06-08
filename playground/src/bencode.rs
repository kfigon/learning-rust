#[test]
fn encode_int_test() {
    assert_eq!("i45e", encode_int(45));
}

#[test]
fn decode_int_test() {
    assert_eq!(Ok(45), decode_int("i45e"));
    assert_eq!(Ok(-123), decode_int("i-123e"));
    assert_eq!(Err(ErrorMsg("missing end or beginning tags")), decode_int("123"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_int("i"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_int("i1"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_int("ie"));
    assert_eq!(Err(ErrorMsg("missing end or beginning tags")), decode_int("12e"));
    assert_eq!(Err(ErrorMsg("cant parse")), decode_int("iasdfe"));
}

#[test]
fn encode_str_test() {
    assert_eq!("3:foo", encode_str("foo"));
    assert_eq!("4:1234", encode_str("1234"));
    assert_eq!("5:4:foo", encode_str("4:foo"));
}

#[test]
fn decode_str_test() {
    assert_eq!(Ok("foo".to_owned()), decode_str("3:foo"));
    assert_eq!(Ok("asdfgqwe123r".to_owned()), decode_str("12:asdfgqwe123r"));
    assert_eq!(Ok("1234".to_owned()), decode_str("4:1234"));
    assert_eq!(Ok("4:foo".to_owned()), decode_str("5:4:foo"));
    assert_eq!(Ok("4:fo".to_owned()), decode_str("4:4:foo"));
    assert_eq!(Err(ErrorMsg("missing len")), decode_str("0:asd"));
    assert_eq!(Err(ErrorMsg("missing len")), decode_str("-1:"));
    assert_eq!(Err(ErrorMsg("missing len")), decode_str("0"));
    assert_eq!(Err(ErrorMsg("invalid len")), decode_str("6:4:foo"));
    assert_eq!(Err(ErrorMsg("missing len")), decode_str("asd"));
    assert_eq!(Err(ErrorMsg("missing len")), decode_str("4asd"));
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
    match (chars.next(), chars.next_back()) {
        (Some('i'), Some('e')) => (),
        _ => return Err(ErrorMsg("missing end or beginning tags")),
    }

    chars.as_str().parse::<i32>().map_err(|_| ErrorMsg("cant parse"))
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
        if let Some(c) = chars.next() {
            out.push(c);
        } else {
            return Err(ErrorMsg("invalid len"));
        }
    }

    Ok(out)
}