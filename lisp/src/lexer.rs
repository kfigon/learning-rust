use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Opening{line: usize},
    Closing{line: usize},
    Literal{line: usize, v: Literal},
    Identifier{line: usize, v: String},
    Invalid{line: usize, v: String},
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(i32),
    String(String),
    Boolean(bool),
}


pub fn lex(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut out = Vec::new();
    let mut line_number = 1;

    while let Some(current) = chars.next() {
        let tok = match current {
            c if c.is_whitespace() => {
                if current == '\n' {
                    line_number += 1;
                }
                None
            },
            ')' => Some(Token::Closing{line: line_number}),
            '(' => Some(Token::Opening{line: line_number}),
            '"' => {
                let word = current.to_string() + &read_until(&mut chars, |c| c != '"');
                match chars.peek() {
                    Some('"') => {
                        chars.next();
                        Some(Token::Literal { line: line_number, v: Literal::String(word + "\"")})
                    },
                    _ => Some(Token::Invalid{line: line_number, v: word})
                }
            },
            other => {
                let word = other.to_string() + &read_until(&mut chars, |c| !c.is_whitespace() && c != ')' && c !='(' && c != '"');
                match word.parse::<i32>() {
                    Ok(num) => Some(Token::Literal { line: line_number, v: Literal::Number(num)}),
                    _ => match word.parse::<bool>() {
                        Ok(v) => Some(Token::Literal { line: line_number, v: Literal::Boolean(v)}),
                        Err(_) => Some(Token::Identifier{line: line_number, v:word})
                    }
                }
            }
        };
    
        if let Some(t) = tok {
            out.push(t);
        }
    }
    out
}

// &mut Peekable<impl Iterator<Item = char>>
fn read_until(chars: &mut Peekable<Chars>, fun: impl Fn(char) -> bool) -> String {
    let mut out = String::new();

    while let Some(&next) = chars.peek() {
        if !fun(next) {
            break;
        }
        out.push(next);
        chars.next();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn lex_basic_tokens() {
        let input = "(define somevalue 10)
        (+ 3 (* somevalue somevalue))";
        let expected = vec![
            Token::Opening{line: 1},
            Token::Identifier{ line: 1, v: s("define") },
            Token::Identifier{ line: 1, v: s("somevalue")},
            Token::Literal { line: 1, v: Literal::Number(10) },
            Token::Closing{line: 1},

            Token::Opening{line: 2},
            Token::Identifier{ line: 2, v: s("+")},
            Token::Literal { line: 2, v: Literal::Number(3) },
            Token::Opening{line: 2},
            Token::Identifier{ line: 2, v: s("*")},
            Token::Identifier{ line: 2, v: s("somevalue")},
            Token::Identifier{ line: 2, v: s("somevalue")},
            Token::Closing { line: 2 },
            Token::Closing { line: 2 },
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_identifiers() {
        let input = " somevalue definee";
        let expected = vec![
            Token::Identifier{line: 1, v: s("somevalue")},
            Token::Identifier{line: 1, v: s("definee")},
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_number() {
        let input = " 1234";
        let expected = vec![Token::Literal { line: 1, v: Literal::Number(1234)}];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_whitespaces() {
        let input = " \t \n 123\t";
        let expected = vec![Token::Literal { line: 2, v: Literal::Number(123)}];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_whitespaces_and_string() {
        let input = " \t \n \" fo\no\t\"\t";
        let expected = vec![Token::Literal{line: 2, v: Literal::String(s("\" fo\no\t\""))}];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_string_literal() {
        let input = "\" hello world if 123\" 123";

        let expected = vec![
            Token::Literal { line: 1, v: Literal::String(s("\" hello world if 123\""))},
            Token::Literal{line: 1, v: Literal::Number(123)},
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_invalid_string() {
        let input = "\" hello world ";

        let expected = vec![Token::Invalid{line: 1, v: s("\" hello world ")}];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_invalid_string_on_line_3() {
        let input = "
        (define x 3)
        (define s \" hello world ";

        let expected = vec![
            Token::Opening{line: 2},
            Token::Identifier { line: 2, v: s("define")},
            Token::Identifier{line: 2, v: "x".to_owned()},
            Token::Literal { line: 2, v: Literal::Number(3)},
            Token::Closing{line: 2},
            Token::Opening{line: 3},
            Token::Identifier { line: 3, v: s("define")},
            Token::Identifier{line: 3, v: "s".to_owned()},
            Token::Invalid{line: 3, v: "\" hello world ".to_owned()},
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
            Token::Opening{line: 1},
            Token::Identifier{line: 1, v: "define".to_owned()},
            Token::Identifier{line: 1, v: "apples".to_owned()},
            Token::Literal{line: 1, v: Literal::Number(5)},
            Token::Closing{line: 1},
            Token::Opening{line: 2},
            Token::Identifier{line: 2, v: s("define")},
            Token::Identifier{line: 2, v: s("oranges")},
            Token::Literal{line: 2, v: Literal::Number(6)},
            Token::Closing{line: 2},
            Token::Opening{line: 3},
            Token::Identifier{line: 3, v: s("if")},
            Token::Opening{line: 3},
            Token::Identifier{line: 3, v: "<=".to_owned()},
            Token::Identifier{line: 3, v: "apples".to_owned()},
            Token::Identifier{line: 3, v: "oranges".to_owned()},
            Token::Closing{line: 3},
            
            Token::Opening{line: 4},
            Token::Identifier{line: 4, v: "printf".to_owned()},
            Token::Literal{line: 4, v: Literal::String(s("\"Apples\""))},
            Token::Closing{line: 4},
            
            Token::Opening{line: 5},
            Token::Identifier{line: 5, v: "printf".to_owned()},
            Token::Literal{line: 5, v: Literal::String(s("\"Oranges\""))},
            Token::Closing{line: 5},
            Token::Closing{line: 5},
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_operators() {
        let input = "< <= > >= ! !! !=";

        let expected = vec![
            Token::Identifier { line: 1, v: s("<")},
            Token::Identifier { line: 1, v: s("<=")},
            Token::Identifier { line: 1, v: s(">")},
            Token::Identifier { line: 1, v: s(">=")},
            Token::Identifier { line: 1, v: s("!")},
            Token::Identifier { line: 1, v: s("!!")},
            Token::Identifier { line: 1, v: s("!=")},
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_operators_without_spaces() {
        let input = "<<=>>=!!!!=";

        let expected = vec![
            Token::Identifier{line: 1, v: s("<<=>>=!!!!=")},
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_function() {
        let input = "(define (dbl x)
                            (* 2 x))
    
                            (dbl 2)";
        let expected = vec![
            Token::Opening{line: 1},
            Token::Identifier{line: 1, v: "define".to_string()},
            Token::Opening{line: 1},
            Token::Identifier{line: 1, v: "dbl".to_string()},
            Token::Identifier{line: 1, v: "x".to_string()},
            Token::Closing{line: 1},

            Token::Opening{line: 2},
            Token::Identifier{line: 2, v: "*".to_string()},
            Token::Literal{line: 2, v: Literal::Number(2)},
            Token::Identifier{line: 2, v: "x".to_string()},
            Token::Closing{line: 2},
            Token::Closing{line: 2},
            
            Token::Opening{line: 4},
            Token::Identifier{line: 4, v: "dbl".to_string()},
            Token::Literal{ line: 4, v: Literal::Number(2)},
            Token::Closing{line: 4},
        ];
        assert_eq!(lex(input), expected)
    }

    #[test]
    fn lex_boolean() {
        let input = "(define x true)
                           (define y false)
                           (define z(= x y))";
        let expected = vec![
            Token::Opening{line: 1},
            Token::Identifier{line: 1, v: "define".to_string()},
            Token::Identifier{line: 1, v: "x".to_string()},
            Token::Literal{line: 1, v: Literal::Boolean(true)},
            Token::Closing{line: 1},

            Token::Opening{line: 2},
            Token::Identifier{line: 2, v: "define".to_string()},
            Token::Identifier{line: 2, v: "y".to_string()},
            Token::Literal{line: 2, v: Literal::Boolean(false)},
            Token::Closing{line: 2},

            Token::Opening{line: 3},
            Token::Identifier{line: 3, v: "define".to_string()},
            Token::Identifier{line: 3, v: "z".to_string()},
            Token::Opening{line: 3},
            Token::Identifier{line: 3, v: "=".to_string()},
            Token::Identifier{line: 3, v: "x".to_string()},
            Token::Identifier{line: 3, v: "y".to_string()},
            Token::Closing{line: 3},
            Token::Closing{line: 3},
        ];
        assert_eq!(lex(input), expected)
    }
}
