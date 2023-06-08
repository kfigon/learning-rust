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


fn encode_int(i: i32) -> String {
    format!("i{i}e")
}

#[derive(Debug, PartialEq, Eq)]
struct ErrorMsg(&'static str);
fn decode_int(i: &str) -> Result<i32, ErrorMsg> {
    if i.len() < 3 {
        return Err(ErrorMsg("invalid len"));
    }
    let mut chars = i.chars();
    match (chars.nth(0), chars.nth_back(0)) {
        (Some('i'), Some('e')) => (),
        _ => return Err(ErrorMsg("missing end or beginning tags")),
    }

    i[1..i.len()-1].parse::<i32>().map_err(|_| ErrorMsg("cant parse"))
}