use std::collections::HashMap;

use crate::parser::{CompilerError, SExpression, Atom};

struct Evaluator {
    env: Env
}

impl Evaluator {
    fn new() -> Self {
        Self { env: Env::std_env() }
    }

    fn eval_expr(&mut self, e: SExpression) -> Result<SExpression, CompilerError> {
        match &e {
            SExpression::Atom(v) => Ok(SExpression::Atom(v.clone())),
            SExpression::List(v) => {
                let symbol = v.get(0).unwrap();
                match symbol {
                    SExpression::Atom(at) => match at {
                        Atom::Identifier(id) => {
                            let fun = self.env.env.get(id).unwrap();
                            let r = fun(&e);
                            Ok(r)
                        },
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            },
        }
    }
}

struct Env {
    env: HashMap<String, Box<dyn Fn(&SExpression)->SExpression>>
}

impl Env {
    fn std_env() -> Self {
        let x: Vec<(String, Box<dyn Fn(&SExpression) -> SExpression>)> = vec![
            ("+".to_string(), Box::new(plus)),
            ("-".to_string(), Box::new(minus)),
        ];

        let e = HashMap::from_iter(x);

        Self { 
            env: e
        }
    }
}

fn plus(e: &SExpression) -> SExpression {
    match e {
        SExpression::Atom(_) => todo!(),
        SExpression::List(v) => {
            let args = &v[1..];
            let mut out = 0;
            for a in args {
                match a {
                    SExpression::Atom(a) => match a {
                        crate::parser::Atom::Number(v) => out += *v,
                        _ => todo!()
                    },
                    _ => todo!()
                }
            }
            return SExpression::Atom(Atom::Number(out));
        },
    }
}

fn minus(e: &SExpression) -> SExpression {
    todo!()
}

pub fn eval(ast: Vec<SExpression>) -> Result<Vec<SExpression>, CompilerError> {
    let mut evaluator = Evaluator::new();
    let mut out = vec![];
    
    for e in ast {
        out.push(evaluator.eval_expr(e)?);
    }

    Ok(out)
}

#[cfg(test)]
mod test {
    use crate::{parser::{parse, Atom}, lexer::lex};

    use super::*;

    fn run(input: &str) -> Vec<SExpression> {
        eval(parse(lex(input)).unwrap()).unwrap()
    }

    #[test]
    fn eval_plus() {
        let r = run("(+ 1 2)");
        assert_eq!(r, vec![
            SExpression::Atom(Atom::Number(3)),
        ])
    }
}