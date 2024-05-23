use std::{iter::Peekable, str::Chars};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex1() {
        let input = "let xd = 15 + 1;";
        let expected = vec![
            Token::Keyword("let".to_string()),
            Token::Identifier("xd".to_string()),
            Token::Operator("=".to_string()),
            Token::Number("15".to_string()),
            Token::Operator("+".to_string()),
            Token::Number("1".to_string()),
            Token::Semicolon(";".to_string()),
        ];
        assert_eq!(lexer(input), expected);
    }

    #[test]
    fn lex2() {
        let input = "let xd=15 ==31);";
        let expected = vec![
            Token::Keyword("let".to_string()),
            Token::Identifier("xd".to_string()),
            Token::Operator("=".to_string()),
            Token::Number("15".to_string()),
            Token::Operator("==".to_string()),
            Token::Number("31".to_string()),
            Token::Closing(")".to_string()),
            Token::Semicolon(";".to_string()),
        ];
        assert_eq!(lexer(input), expected);
    }
}

#[derive(PartialEq, Debug)]
enum Token {
    Opening(String),
    Closing(String),
    Operator(String),
    Keyword(String),
    Identifier(String),
    Number(String),
    Semicolon(String),
}

fn lexer(input: &str) -> Vec<Token> {
    let mut out = Vec::<Token>::new();
    let mut chars = input.chars().peekable();
    let is_keyword = |word: &String| word == "if" || word == "for" || word == "let";

    while let Some(current_char) = chars.next() {
        if current_char.is_whitespace() {
            continue;
        } else if current_char == ';' {
            out.push(Token::Semicolon(current_char.to_string()));
        } else if current_char == '(' || current_char == '{' {
            out.push(Token::Opening(current_char.to_string()));
        } else if current_char == ')' || current_char == '}' {
            out.push(Token::Closing(current_char.to_string()));
        } else if current_char == '+'
            || current_char == '-'
            || current_char == '/'
            || current_char == '*'
        {
            out.push(Token::Operator(current_char.to_string()));
        } else if current_char == '=' {
            let word = read_until(&mut chars, current_char, |c| c == '=');
            out.push(Token::Operator(word)); // == or =
        } else if current_char.is_ascii_digit() {
            let num = read_until(&mut chars, current_char, |c| c.is_ascii_digit());
            out.push(Token::Number(num));
        } else {
            let word = read_until(&mut chars, current_char, |c| c.is_alphabetic());
            if is_keyword(&word) {
                out.push(Token::Keyword(word));
            } else {
                out.push(Token::Identifier(word));
            }
        }
    }

    out
}

fn read_until<F>(chars: &mut Peekable<Chars>, current_char: char, fun: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut out = String::new();
    out.push(current_char);

    while let Some(c) = chars.peek() {
        if fun(*c) {
            out.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    out
}
