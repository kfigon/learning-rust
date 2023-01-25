use std::{iter::Peekable, slice::IterMut};

use crate::lexer::{Token};

#[derive(Debug, PartialEq)]
pub struct Ast(Vec<SExpression>);

#[derive(Debug, PartialEq)]
pub enum SExpression {
}

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidToken(String),
}

struct Parser {
    errors: Vec<CompilerError>,
    expressions: Vec<SExpression>,
    tokens: Vec<Token>,
    current_idx: usize, // todo: use iterators, instead of idx. I can't do it because I don't know ownership that good
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { errors: vec![],
             expressions: vec![],
             tokens: tokens,
             current_idx: 0
        }
    }

    fn consume(&mut self) {
        self.current_idx += 1;
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_idx)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_idx+1)
    }

    fn run(&mut self) {
        while let Some(current) = self.current_token() {
            match current {
                Token::Invalid(line, t) => {
                    self.errors.push(
                        CompilerError::InvalidToken(
                            format!("Invalid token on {line}: {}", t.to_owned())
                        ));
                    break;
                }
                Token::Opening(_) => todo!(), // self.parse_expression(&mut tokens), 
                Token::Closing(_) => todo!(),
                Token::Operator(_) => todo!(),
                Token::Number(_) => todo!(),
                Token::Keyword(_) => todo!(),
                Token::Identifier(_) => todo!(),
                Token::String(_) => todo!(),
                Token::Boolean(_) => todo!(),
            }
            self.consume();
        }
    }

    fn parse_expression(&mut self) {

    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, Vec<CompilerError>> {
    let mut p = Parser::new(tokens);
    p.run();
    if p.errors.len() != 0 {
        return Err(p.errors);
    }
    Ok(Ast(p.expressions))
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use super::*;

    fn compile(input: &str) -> Result<Ast, Vec<CompilerError>> {
        parse(lex(input))
    }

    #[test]
    fn invalid_token() {
        let tok = "(define x \"invalid string";
        assert_eq!(
            compile(tok),
            Err(vec![CompilerError::InvalidToken("\"invalid string".to_string())])
        )
    }

    #[test]
    fn parse_simple_math() {
        todo!();
        let tok = "(+ 3 1)";
        assert_eq!(
            compile(tok),
            Ok(Ast(vec![]))
        )
    }
}