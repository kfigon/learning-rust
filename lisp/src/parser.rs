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
        let current = self.current_token().unwrap();

        match current {
            Token::Invalid(line, t) => return Err(self.invalid_token_error(*line, t.clone())),
            Token::Opening(_) => return self.parse_expression(),
            Token::Closing(_) => todo!(),
            Token::Operator(_) => return Ok(SExpression::Atom(current.clone())),
            Token::Number(_) => return Ok(SExpression::Atom(current.clone())),
            Token::Keyword(_) => return Ok(SExpression::Atom(current.clone())),
            Token::Identifier(_) => return Ok(SExpression::Atom(current.clone())),
            Token::String(_) => return Ok(SExpression::Atom(current.clone())),
            Token::Boolean(_) => return Ok(SExpression::Atom(current.clone())),
        }
    }

    pub fn run(&mut self) {
        match self.parse() {
            Ok(v) => self.expressions.push(v),
            Err(v) => self.errors.push(v),
        }
    }

    fn parse_expression(&mut self) -> Result<SExpression, CompilerError> {
        self.consume(); // (
        let mut list: Vec<SExpression> = vec![];
        while let Some(current) = self.current_token() {
            if let Token::Closing(_) = current {
                self.consume();
                return Ok(SExpression::List(list));
            }
            let v = self.parse()?;
            list.push(v);
            self.consume();
        }
        return Err(CompilerError::IncompleteExpression("Found unmatched list".to_string()));
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
}