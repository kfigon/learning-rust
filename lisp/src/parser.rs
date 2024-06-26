use std::{iter::Peekable};

use crate::lexer::{Token, self};

#[derive(Debug, PartialEq, Clone)]
pub enum SExpression {
    Void,
    Number(i32),
    Boolean(bool),
    String(String),
    Identifier(String),
    List(Vec<SExpression>)
}

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidToken(lexer::Token),
    IncompleteExpression(lexer::Token),
    UnexpectedEof,
    UnknownSymbol(String),
    InvalidList(SExpression),
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

    fn parse(mut self) -> Result<Vec<SExpression>, Vec<CompilerError>> {
        while let Some(tok) = self.tokens.next() {
            match tok {
                Token::Opening { line } => {
                    match self.parse_exp() {
                        Ok(v) => self.expressions.push(SExpression::List(v)),
                        Err(e) => self.errors.push(e),
                    }
                }
                v => self.errors.push(CompilerError::InvalidToken(v)),
            }
        }

        if !self.errors.is_empty() {
            Err(self.errors)
        } else {
            Ok(self.expressions)
        }
    }

    fn parse_exp(&mut self) -> Result<Vec<SExpression>, CompilerError> {
        let mut elems = vec![];
        while let Some(next) = self.tokens.next() {
            match next {
                Token::Closing { line } => break,
                Token::Invalid { line, ref v } => return Err(CompilerError::InvalidToken(next)), // string is ref type, so I couldn't just move here. ref is required to not partially move `next`` to `v`
                Token::Identifier { line, v } => elems.push(SExpression::Identifier(v)),
                Token::Literal { line, v } => match v {
                    lexer::Literal::Number(n) => elems.push(SExpression::Number(n)),
                    lexer::Literal::String(s) => elems.push(SExpression::String(s)),
                    lexer::Literal::Boolean(b) => elems.push(SExpression::Boolean(b)),
                },
                Token::Opening { line } => elems.push(SExpression::List(self.parse_exp()?)),
            }
        }
        Ok(elems)
    }

}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<SExpression>, Vec<CompilerError>> {
    let mut p = Parser::new(tokens.into_iter());
    p.parse()
}

pub fn eval(ast: Vec<SExpression>) {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use super::*;

    fn s(x: &str) -> String {
        x.to_string()
    }

    fn compile(input: &str) -> Result<Vec<SExpression>, Vec<CompilerError>> {
        parse(lex(input))
    }

    #[test]
    fn single_exp() {
        let input = "(+ 1 2)";
        let ast = compile(input).unwrap();
        assert_eq!(ast, vec![
            SExpression::List(vec![
                SExpression::Identifier(s("+")),
                SExpression::Number(1),
                SExpression::Number(2),
            ])
        ]);
    }

    #[test]
    fn two_exp() {
        let input = "(+ 1 2)
        (- 3 4)";
        let ast = compile(input).unwrap();
        assert_eq!(ast, vec![
            SExpression::List(vec![
                SExpression::Identifier(s("+")),
                SExpression::Number(1),
                SExpression::Number(2),
            ]),
            SExpression::List(vec![
                SExpression::Identifier(s("-")),
                SExpression::Number(3),
                SExpression::Number(4),
            ])
        ]);
    }

    #[test]
    fn nested_expr() {
        let input = "(+ 1 (* 2 4))";
        let ast = compile(input).unwrap();
        assert_eq!(ast, vec![
            SExpression::List(vec![
            SExpression::Identifier(s("+")),
                SExpression::Number(1),
                SExpression::List(vec![
                    SExpression::Identifier(s("*")),
                    SExpression::Number(2),
                    SExpression::Number(4),
                ]),
            ])
        ]);   
    }
}