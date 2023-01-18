use std::{iter::Peekable, str::Chars};


#[derive(Debug, PartialEq)]
pub enum Token {
    Opening(String),
    Closing(String),
    Operator(String),
    Number(String),
    Keyword(String),
    Identifier(String),
    
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut out = Vec::new();

    let is_keyword = |s: &String| s == "define";

    while let Some(current) = chars.next() {
        if current.is_whitespace() {
            continue;
        } else if current == ')' {
            out.push(Token::Closing(current.to_string()));
        } else if current == '(' {
            out.push(Token::Opening(current.to_string()));
        } else if current == '+' || current == '-' || current == '*' || current == '/' || current == '=' {
            out.push(Token::Operator(current.to_string()));
        } else if current.is_ascii_digit() {
            let num = read_until(&mut chars, current, |c| c.is_ascii_digit());
            out.push(Token::Number(num));
        } else {
            let word = read_until(&mut chars, current, |c| c.is_alphanumeric());
            if is_keyword(&word) {
                out.push(Token::Keyword(word));
            } else {
                out.push(Token::Identifier(word));
            }
        }
    }
    out
}

fn read_until<F>(chars: &mut Peekable<Chars>, current: char, fun: F) -> String 
 where F: Fn(&char) -> bool {
    let mut out = String::new();
    out.push(current);

    while let Some(next) = chars.peek() {
        if fun(&next) {
            out.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex1() {
        let input = "(define somevalue 10)
        (+ 3 (* somevalue somevalue))";
        let expected = vec![
            Token::Opening("(".to_owned()),
            Token::Keyword("define".to_owned()),
            Token::Identifier("somevalue".to_owned()),
            Token::Number("10".to_owned()),
            Token::Closing(")".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Operator("+".to_owned()),
            Token::Number("3".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Operator("*".to_owned()),
            Token::Identifier("somevalue".to_owned()),
            Token::Identifier("somevalue".to_owned()),
            Token::Closing(")".to_owned()),
            Token::Closing(")".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex2() {
        let input = " somevalue";
        let expected = vec![
            Token::Identifier("somevalue".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex3() {
        let input = " 1234";
        let expected = vec![
            Token::Number("1234".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }
}
