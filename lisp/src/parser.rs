use crate::lexer::{Token};

#[derive(Debug, PartialEq)]
pub struct Ast(Vec<SExpression>);

#[derive(Debug, PartialEq)]
pub enum SExpression {
    Atom(Token),
    List(Vec<SExpression>)
}

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidToken(String),
    IncompleteExpression(String),
    UnexpectedEof,
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

    fn invalid_token_error(&self, line: usize, token: String) -> CompilerError {
        CompilerError::InvalidToken(
                format!("Invalid token on {line}: {}", token))
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