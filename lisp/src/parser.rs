use std::iter::Peekable;

use crate::lexer::{Token, self};

#[derive(Debug, PartialEq)]
pub struct Ast(Vec<SExpression>);

#[derive(Debug, PartialEq)]
pub enum SExpression {
    Atom(Token),
    List(Vec<SExpression>)
}

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidToken(lexer::Token),
    IncompleteExpression(lexer::Token),
    UnexpectedEof,
}


struct Parser<T: Iterator<Item = Token>> {
    errors: Vec<CompilerError>,
    expressions: Vec<SExpression>,
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    fn new(tokens: T) -> Self {
        Parser { errors: vec![],
             expressions: vec![],
             tokens: tokens.peekable(),
        }
    }

    fn parse(&mut self) -> Result<SExpression, CompilerError> {
        todo!()
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, Vec<CompilerError>> {
    todo!()
}

pub fn eval(ast: Ast) -> Vec<Token> {
    let mut out = Vec::<Token>::new();
    for t in ast.0 {
        out.push(eval_expression(t));
    }
    out
}

fn eval_expression(e: SExpression) -> Token {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use super::*;

    fn compile(input: &str) -> Result<Ast, Vec<CompilerError>> {
        parse(lex(input))
    }
}