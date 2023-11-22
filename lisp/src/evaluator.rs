use std::collections::HashMap;

use crate::parser::{CompilerError, SExpression};

struct Evaluator {
    env: Env
}

impl Evaluator {
    fn new() -> Self {
        Self { env: Env::std_env() }
    }

    fn eval_expr(&mut self, e: SExpression) -> Result<SExpression, CompilerError> {
        match e {
            SExpression::Void => Ok(e),
            SExpression::Number(_) => Ok(e),
            SExpression::Boolean(_) => Ok(e),
            SExpression::String(_) => Ok(e),
            SExpression::Identifier(_) => Ok(e),
            SExpression::List(ref v) => {
                let first = v.get(0);
                let first = match first {
                    Some(s) => s,
                    None => return Ok(SExpression::Void),
                };

                match first {
                    SExpression::Void => Err(CompilerError::InvalidList(e)),
                    SExpression::Number(_) => Err(CompilerError::InvalidList(e)),
                    SExpression::Boolean(_) => Err(CompilerError::InvalidList(e)),
                    SExpression::String(_) => Err(CompilerError::InvalidList(e)),
                    SExpression::Identifier(id) => {
                            if id == "+" {
                                return self.plus(e);
                            }
                            return Err(CompilerError::UnknownSymbol(id.clone()));
                            // let func = self.env.env.get(id);
                            // let func = match func {
                            //     Some(s) => s,
                            //     None => return Err(CompilerError::UnknownSymbol(id.clone())),
                            // };

                            // Ok(func(self, e))
                        },
                    SExpression::List(_) => self.eval_expr(first.clone()),
                }
            }
        }
    }

    fn plus(&mut self, e: SExpression) -> Result<SExpression, CompilerError> {
        match e {
            SExpression::List(v) => {
                let args = &v[1..];
                let mut out = 0;
                for a in args {
                    match a {
                        SExpression::Number(n) => out += *n,
                        _ => {
                            let res = self.eval_expr(a.clone())?;
                            match res {
                                SExpression::Number(n) => out += n,
                                _ => return Err(CompilerError::InvalidList(res)),
                            }
                        }
                    }
                }
                return Ok(SExpression::Number(out));
            },
            SExpression::Identifier(_) => Err(CompilerError::InvalidList(e)),
            SExpression::Void => Err(CompilerError::InvalidList(e)),
            SExpression::Number(_) => Err(CompilerError::InvalidList(e)),
            SExpression::Boolean(_) => Err(CompilerError::InvalidList(e)),
            SExpression::String(_) => Err(CompilerError::InvalidList(e)),
        }
    }
}

pub fn eval(ast: Vec<SExpression>) -> Result<Vec<SExpression>, CompilerError> {
    let mut evaluator = Evaluator::new();
    let mut out = vec![];
    
    for e in ast {
        out.push(evaluator.eval_expr(e)?);
    }

    Ok(out)
}


struct Env {
    env: HashMap<String, Box<dyn Fn(&Evaluator,SExpression)->SExpression>>
}

impl Env {
    fn std_env() -> Self {
        // let x: Vec<(String, Box<dyn Fn(&Evaluator, SExpression) -> SExpression>)> = vec![
        //     ("+".to_string(), Box::new(plus)),
        //     ("-".to_string(), Box::new(minus)),
        // ];

        Self { 
            env: HashMap::new()//from_iter(x)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{parser::{parse}, lexer::lex};

    use super::*;

    fn run(input: &str) -> Vec<SExpression> {
        eval(parse(lex(input)).unwrap()).unwrap()
    }

    #[test]
    fn eval_plus() {
        let r = run("(+ 1 2)");
        assert_eq!(r, vec![
            SExpression::Number(3),
        ])
    }

    #[test]
    fn eval_plus_nested() {
        let r = run("(+ (+ 4 5) 2)");
        assert_eq!(r, vec![
            SExpression::Number(11),
        ])
    }
}