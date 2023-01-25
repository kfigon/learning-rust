use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Opening(String),
    Closing(String),
    Operator(String),
    Number(i32),
    Boolean(bool),
    Keyword(String),
    Identifier(String),
    String(String),
    Invalid(usize, String), // todo: make it a lexer error and return early
}


pub fn lex(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut out = Vec::new();
    let mut line_number = 1;

    let is_keyword = |s: &String| s == "define" || s == "if";
    let single_char_operator = |c: char| c == '+' || c == '-' || c == '*' || c == '/' || c == '=';
    let multi_char_operator = |c: char| c == '!' || c == '<' || c == '>';

    while let Some(current) = chars.next() {
        if current.is_whitespace() {
            if current == '\n' {
                line_number += 1;
            }
            continue;
        } else if current == ')' {
            out.push(Token::Closing(current.to_string()));
        } else if current == '(' {
            out.push(Token::Opening(current.to_string()));
        } else if single_char_operator(current) {
            out.push(Token::Operator(current.to_string()));
        } else if multi_char_operator(current) {
            let op = if let Some('=') = chars.peek() {
                chars.next();
                current.to_string() + "="
            } else {
                current.to_string()
            };
            out.push(Token::Operator(op));
        } else if current == '"' {
            let word = read_until(&mut chars, current, |c| *c != '"');
            if let Some('"') = chars.peek() {
                chars.next();
                out.push(Token::String(word + "\""));
            } else {
                out.push(Token::Invalid(line_number, word));
            }
        } else if current.is_ascii_digit() {
            let num = read_until(&mut chars, current, |c| c.is_ascii_digit());
            match num.parse() {
                Ok(v) => out.push(Token::Number(v)),
                Err(_) => out.push(Token::Invalid(line_number, num)),
            }
        } else {
            let word = read_until(&mut chars, current, |c| c.is_alphanumeric());
            if is_keyword(&word) {
                out.push(Token::Keyword(word));
            } else if word == "false" || word == "true" {
                match word.parse() {
                    Ok(v) => out.push(Token::Boolean(v)),
                    Err(_) => out.push(Token::Invalid(line_number, word)),
                }
            } else {
                out.push(Token::Identifier(word));
            }
        }
    }
    out
}

fn read_until<F>(chars: &mut Peekable<Chars>, current: char, fun: F) -> String
where
    F: Fn(&char) -> bool,
{
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
    fn lex_basic_tokens() {
        let input = "(define somevalue 10)
        (+ 3 (* somevalue somevalue))";
        let expected = vec![
            Token::Opening("(".to_owned()),
            Token::Keyword("define".to_owned()),
            Token::Identifier("somevalue".to_owned()),
            Token::Number(10),
            Token::Closing(")".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Operator("+".to_owned()),
            Token::Number(3),
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
    fn lex_identifiers() {
        let input = " somevalue definee";
        let expected = vec![
            Token::Identifier("somevalue".to_owned()),
            Token::Identifier("definee".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_number() {
        let input = " 1234";
        let expected = vec![Token::Number(1234)];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_whitespaces() {
        let input = " \t \n 123\t";
        let expected = vec![Token::Number(123)];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_whitespaces_and_string() {
        let input = " \t \n \" fo\no\t\"\t";
        let expected = vec![Token::String("\" fo\no\t\"".to_string())];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_string_literal() {
        let input = "\" hello world if 123\" 123";

        let expected = vec![
            Token::String("\" hello world if 123\"".to_owned()),
            Token::Number(123),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_invalid_string() {
        let input = "\" hello world ";

        let expected = vec![Token::Invalid(1, "\" hello world ".to_owned())];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_invalid_string_on_line_3() {
        let input = "
        (define x 3)
        (define s \" hello world ";

        let expected = vec![
            Token::Opening("(".to_owned()),
            Token::Keyword("define".to_owned()),
            Token::Identifier("x".to_owned()),
            Token::Number(3),
            Token::Closing(")".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Keyword("define".to_owned()),
            Token::Identifier("s".to_owned()),
            Token::Invalid(3, "\" hello world ".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_if_statement() {
        let input = "(define apples 5)
        (define oranges 6)
        (if (<= apples oranges)
            (printf \"Apples\")
            (printf \"Oranges\"))";

        let expected = vec![
            Token::Opening("(".to_owned()),
            Token::Keyword("define".to_owned()),
            Token::Identifier("apples".to_owned()),
            Token::Number(5),
            Token::Closing(")".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Keyword("define".to_owned()),
            Token::Identifier("oranges".to_owned()),
            Token::Number(6),
            Token::Closing(")".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Keyword("if".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Operator("<=".to_owned()),
            Token::Identifier("apples".to_owned()),
            Token::Identifier("oranges".to_owned()),
            Token::Closing(")".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Identifier("printf".to_owned()),
            Token::String("\"Apples\"".to_owned()),
            Token::Closing(")".to_owned()),
            Token::Opening("(".to_owned()),
            Token::Identifier("printf".to_owned()),
            Token::String("\"Oranges\"".to_owned()),
            Token::Closing(")".to_owned()),
            Token::Closing(")".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_operators() {
        let input = "< <= > >= ! !! !=";

        let expected = vec![
            Token::Operator("<".to_owned()),
            Token::Operator("<=".to_owned()),
            Token::Operator(">".to_owned()),
            Token::Operator(">=".to_owned()),
            Token::Operator("!".to_owned()),
            Token::Operator("!".to_owned()),
            Token::Operator("!".to_owned()),
            Token::Operator("!=".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_operators_without_spaces() {
        let input = "<<=>>=!!!!=";

        let expected = vec![
            Token::Operator("<".to_owned()),
            Token::Operator("<=".to_owned()),
            Token::Operator(">".to_owned()),
            Token::Operator(">=".to_owned()),
            Token::Operator("!".to_owned()),
            Token::Operator("!".to_owned()),
            Token::Operator("!".to_owned()),
            Token::Operator("!=".to_owned()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_function() {
        let input = "(define (dbl x)
                            (* 2 x))
    
                            (dbl 2)";
        let expected = vec![
            Token::Opening("(".to_string()),
            Token::Keyword("define".to_string()),
            Token::Opening("(".to_string()),
            Token::Identifier("dbl".to_string()),
            Token::Identifier("x".to_string()),
            Token::Closing(")".to_string()),
            Token::Opening("(".to_string()),
            Token::Operator("*".to_string()),
            Token::Number(2),
            Token::Identifier("x".to_string()),
            Token::Closing(")".to_string()),
            Token::Closing(")".to_string()),
            Token::Opening("(".to_string()),
            Token::Identifier("dbl".to_string()),
            Token::Number(2),
            Token::Closing(")".to_string()),
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_boolean() {
        let input = "(define x true)
                           (define y false)
                           (define z(= x y))";
        let expected = vec![
            Token::Opening("(".to_string()),
            Token::Keyword("define".to_string()),
            Token::Identifier("x".to_string()),
            Token::Boolean(true),
            Token::Closing(")".to_string()),

            Token::Opening("(".to_string()),
            Token::Keyword("define".to_string()),
            Token::Identifier("y".to_string()),
            Token::Boolean(false),
            Token::Closing(")".to_string()),

            Token::Opening("(".to_string()),
            Token::Keyword("define".to_string()),
            Token::Identifier("z".to_string()),
            
            Token::Opening("(".to_string()),
            Token::Operator("=".to_string()),
            Token::Identifier("x".to_string()),
            Token::Identifier("y".to_string()),
            Token::Closing(")".to_string()),

            Token::Closing(")".to_string()),
        ];
        assert_eq!(lex(input), expected)
    }
}
