#![allow(dead_code)]

#[test]
fn encode_str_test() {
    assert_eq!("3:yes", encode_str("yes"));
    assert_eq!("7:yes:123", encode_str("yes:123"));
    assert_eq!(Ok("yes:123".to_owned()), decode_str("7:yes:123"));
}

#[test]
fn encode_int_test() {
    assert_eq!("i59e", encode_int(59));
    assert_eq!("i-12e", encode_int(-12));
    // assert_eq!(Ok(-12), decode_int("i-12e"));
    // assert_eq!(Ok(4567), decode_int("i456e"));
}

enum BencodeObject {
    String(String),
    Integer(String),
    Dictionary(String),
    List(String),
}

fn encode_str(s: &str) -> String {
    format!("{}:{}", s.len(), s)
}

#[derive(Debug, PartialEq, Eq)]
struct ErrorMessage(String);

fn decode_str(s: &str) -> Result<String, ErrorMessage> {
    let mut num = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else {
            break;
        }
    }

    let length = match num.parse::<usize>() {
        Ok(v) => v,
        Err(e) => return Err(ErrorMessage(format!("error parsing number, length not found, got: {e}")))
    };

    if s.len() <= num.len() + 1 + length {
        let range = num.len()+1..num.len() + 1 + length;
        let mut out = String::with_capacity(length);
        for i in range {
            out.push(s.chars().nth(i).unwrap());
        }
        return Ok(out);
    }
    Err(ErrorMessage("string to short".to_owned()))
}

fn encode_int(num: i32) -> String {
    format!("i{num}e")
}