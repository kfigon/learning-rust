use crate::lexer::{lex, Token};

#[derive(Debug, PartialEq)]
pub struct Ast;

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidToken(String),
}

pub fn parse(tkns: Vec<Token>) -> Result<Ast, CompilerError> {
    let mut tokens = tkns.iter().peekable();

    while let Some(current) = tokens.next() {
        let next = tokens.peek();

        match current {
            Token::Invalid(line, t) => {
                return Err(CompilerError::InvalidToken(format!(
                    "Invalid token on {line}: {}",
                    t.to_owned()
                )))
            }
            Token::Opening(_) => todo!(),
            Token::Closing(_) => todo!(),
            Token::Operator(_) => todo!(),
            Token::Number(_) => todo!(),
            Token::Keyword(_) => todo!(),
            Token::Identifier(_) => todo!(),
            Token::String(_) => todo!(),
        }
    }

    Ok(Ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compile(input: &str) -> Result<Ast, CompilerError> {
        parse(lex(input))
    }

    #[test]
    fn invalid_token() {
        let tok = "(define x \"invalid string";
        assert_eq!(
            compile(tok),
            Err(CompilerError::InvalidToken("\"invalid string".to_string()))
        )
    }
}