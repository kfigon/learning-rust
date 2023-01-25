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

    fn invalid_token_error(&self, line: usize, token: String) -> CompilerError {
        CompilerError::InvalidToken(
                format!("Invalid token on {line}: {}", token))
    }

    fn parse(&mut self) -> Result<SExpression, CompilerError> {
        let current = self.current_token().ok_or(CompilerError::UnexpectedEof)?;

        match current {
            Token::Invalid(line, t) => Err(self.invalid_token_error(*line, t.clone())),
            Token::Closing(_) => Err(CompilerError::IncompleteExpression("Unexpected closing parenthesis".to_string())),
            Token::Opening(_) => self.parse_expression(),
            Token::Operator(_) => Ok(SExpression::Atom(current.clone())),
            Token::Number(_) => Ok(SExpression::Atom(current.clone())),
            Token::Keyword(_) => Ok(SExpression::Atom(current.clone())),
            Token::Identifier(_) => Ok(SExpression::Atom(current.clone())),
            Token::String(_) => Ok(SExpression::Atom(current.clone())),
            Token::Boolean(_) => Ok(SExpression::Atom(current.clone())),
        }
    }

    fn parse_expression(&mut self) -> Result<SExpression, CompilerError> {
        self.consume(); // (
        let mut list: Vec<SExpression> = vec![];
        while let Some(current) = self.current_token() {
            if let Token::Closing(_) = current {
                self.consume(); // )
                return Ok(SExpression::List(list));
            }
            let v = self.parse()?;
            if let SExpression::List(_) = v {
                // don't consume closing token
            } else {
                self.consume();
            }
            list.push(v);

        }
        Err(CompilerError::IncompleteExpression("Found unmatched list".to_string()))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, Vec<CompilerError>> {
    let mut p = Parser::new(tokens);
    while p.current_idx < p.tokens.len() {
        match p.parse() {
            Ok(v) => {
                println!("Adding exp {:?}", v);
                p.expressions.push(v);
            },
            Err(v) => {
                p.consume();
                println!("error {:?}", v);
                p.errors.push(v)
            },
        }
    }

    if !p.errors.is_empty() {
        return Err(p.errors);
    }
    Ok(Ast(p.expressions))
}

pub fn eval(ast: Ast) -> Token {
    todo!()
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
            Err(vec![CompilerError::InvalidToken("Invalid token on 1: \"invalid string".to_string())])
        )
    }

    #[test]
    fn parse_simple_math() {
        let tok = "(+ 3 1)";
        assert_eq!(
            compile(tok),
            Ok(Ast(vec![
                SExpression::List(vec![
                    SExpression::Atom(Token::Operator("+".to_string())),
                    SExpression::Atom(Token::Number(3)),
                    SExpression::Atom(Token::Number(1)),
                ])
            ]))
        )
    }

    #[test]
    fn parse_simple_math2() {
        let tok = "(+ 3 (* 1 2))";
        assert_eq!(
            compile(tok),
            Ok(Ast(vec![
                SExpression::List(vec![
                    SExpression::Atom(Token::Operator("+".to_string())),
                    SExpression::Atom(Token::Number(3)),
                    SExpression::List(vec![
                        SExpression::Atom(Token::Operator("*".to_string())),
                        SExpression::Atom(Token::Number(1)),
                        SExpression::Atom(Token::Number(2)),
                    ]),
                ])
            ]))
        )
    }

    #[test]
    fn parse_math_with_variable() {
        let tok = "(define somevalue 10)
        (+ 3 (* somevalue somevalue))";
        assert_eq!(
            compile(tok),
            Ok(Ast(vec![
                SExpression::List(vec![
                    SExpression::Atom(Token::Keyword("define".to_string())),
                    SExpression::Atom(Token::Identifier("somevalue".to_string())),
                    SExpression::Atom(Token::Number(10)),
                ]),
                SExpression::List(vec![
                    SExpression::Atom(Token::Operator("+".to_string())),
                    SExpression::Atom(Token::Number(3)),
                    SExpression::List(vec![
                        SExpression::Atom(Token::Operator("*".to_string())),
                        SExpression::Atom(Token::Identifier("somevalue".to_string())),
                        SExpression::Atom(Token::Identifier("somevalue".to_string())),
                    ])
                ])
            ]))
        )
    }

    #[test]
    fn eval_simple_math2() {
        let tok = "(+ 3 (* 1 2))";
        assert_eq!(
            eval(compile(tok).unwrap()),
            Token::Number(5)
        )
    }
}