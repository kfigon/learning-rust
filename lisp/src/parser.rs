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
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { errors: vec![],
             expressions: vec![],
             tokens: tokens,
        }
    }

    fn run(&mut self) {
        let mut tokens  = self.tokens.iter_mut().peekable();

        while let Some(current) = tokens.next() {
            let next = tokens.peek();
            
            match current {
                Token::Invalid(line, t) => {
                    self.errors.push(
                        CompilerError::InvalidToken(
                            format!("Invalid token on {line}: {}", t.to_owned())
                        ));
                    break;
                }
                // here I'm giving up on rust, borrow checker does not allow me to do the mutation
                // and I'm not willing to create an object for each member
                // also I refuse to do functions, as I like to encapsulate interpreter's state when building ast

                // error: second mutable borrow occurs here
                Token::Opening(_) => todo!(), // self.parse_expression(&mut tokens), 
                Token::Closing(_) => todo!(),
                Token::Operator(_) => todo!(),
                Token::Number(_) => todo!(),
                Token::Keyword(_) => todo!(),
                Token::Identifier(_) => todo!(),
                Token::String(_) => todo!(),
                Token::Boolean(_) => todo!(),
            }
        }
    }

    fn parse_expression(&mut self, tokens: &Peekable<IterMut<Token>>) {

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