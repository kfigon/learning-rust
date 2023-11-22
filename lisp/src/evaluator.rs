use std::collections::HashMap;

use crate::parser::{CompilerError, SExpression};

struct Evaluator {
    // todo: store as RefCell?
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
                    SExpression::Identifier(id) if id == "+" => self.plus(e),
                    SExpression::Identifier(id) if id == "=" => self.equal(e),
                    SExpression::Identifier(id) if id == "!=" => self.not_equal(e),
                    SExpression::Identifier(id) if id == "if" => self.if_expression(e),
                    SExpression::Identifier(id) => Err(CompilerError::UnknownSymbol(id.clone())), // todo: env
                    SExpression::List(_) => self.eval_expr(first.clone()),
                }
            }
        }
    }

    fn eval_to_number(&mut self, e: &SExpression) -> Result<i32, CompilerError> {
        match e {
            SExpression::Number(n) => Ok(*n),
            SExpression::Identifier(_) => todo!(),
            _ => {
                let res = self.eval_expr(e.clone())?;
                match res {
                    SExpression::Number(n) => Ok(n),
                    SExpression::Identifier(_) => todo!(),
                    _ => Err(CompilerError::InvalidList(res)),
                }
            }
        }
    }

    fn eval_to_bool(&mut self, e: &SExpression) -> Result<bool, CompilerError> {
        match e {
            SExpression::Boolean(n) => Ok(*n),
            SExpression::Identifier(_) => todo!(),
            _ => {
                let res = self.eval_expr(e.clone())?;
                match res {
                    SExpression::Boolean(n) => Ok(n),
                    SExpression::Identifier(_) => todo!(),
                    _ => Err(CompilerError::InvalidList(res)),
                }
            }
        }
    }

    fn plus(&mut self, e: SExpression) -> Result<SExpression, CompilerError> {
        match e {
            SExpression::List(v) => {
                let mut out = 0;
                for a in &v[1..] {
                    let num = self.eval_to_number(&a)?;
                    out += num;
                }
                Ok(SExpression::Number(out))
            },
            _ => Err(CompilerError::InvalidList(e)),
        }
    }

    fn if_expression(&mut self, e: SExpression) -> Result<SExpression, CompilerError> {
        match e {
            SExpression::List(v) if v.len() == 4 => {
                let condition = self.eval_to_bool(&v[1])?;
                if condition {
                    Ok(v[2].clone())
                } else {
                    Ok(v[3].clone())
                }
            },
            _ => Err(CompilerError::InvalidList(e)),
        }
    }

    fn equal(&mut self, e: SExpression) -> Result<SExpression, CompilerError> {
        match e {
            SExpression::List(ref v) if v.len() == 3 => {
                match (self.eval_expr(v[1].clone())?, self.eval_expr(v[2].clone())?) {
                    (SExpression::Number(a), SExpression::Number(b)) => Ok(SExpression::Boolean(a==b)),
                    (SExpression::Boolean(a), SExpression::Boolean(b)) => Ok(SExpression::Boolean(a==b)),
                    (SExpression::String(a), SExpression::String(b)) => Ok(SExpression::Boolean(a==b)),
                    _ => Err(CompilerError::InvalidList(e.clone()))
                }
            },
            _ => Err(CompilerError::InvalidList(e)),
        }
    }

    fn not_equal(&mut self, e: SExpression) -> Result<SExpression, CompilerError> {
        match self.equal(e.clone())? {
            SExpression::Boolean(a) => Ok(SExpression::Boolean(!a)),
            _ => Err(CompilerError::InvalidList(e)),
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
        assert_eq!(r, vec![SExpression::Number(11)])
    }

    #[test]
    fn eval_plus_nested2() {
        let r = run("(+ (+ 4 (+ 3 2)) 2)");
        assert_eq!(r, vec![SExpression::Number(11)])
    }

    #[test]
    fn if_expression() {
        let r = run(r#"(if (= 1 2) "ok" "not ok")"#);
        assert_eq!(r, vec![SExpression::String("\"not ok\"".to_owned())])
    }

    #[test]
    fn if_expression_2() {
        let r = run(r#"(if (= (+ 1 1) 2) "ok" "not ok")"#);
        assert_eq!(r, vec![SExpression::String("\"ok\"".to_owned())])
    }

    #[test]
    fn comparison_neq() {
        let r = run("(!= 1 2)");
        assert_eq!(r, vec![SExpression::Boolean(true)])
    }

    #[test]
    fn comparison_neq_nested() {
        let r = run("(!= (= 1 1) (= 4 2))");
        assert_eq!(r, vec![SExpression::Boolean(true)])
    }

    #[test]
    fn comparison_eq() {
        let r = run("(= 1 2)");
        assert_eq!(r, vec![SExpression::Boolean(false)])
    }
}