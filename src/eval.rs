use super::ast;

use std::collections::HashMap;
use std::fmt;
use big_s::S;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Value {
    Number(i64),
    Callable(Callable),
    Nil
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => *n != 0,
            _ => true,
        }
    }

    fn into_num(self) -> i64 {
        match self {
            Value::Number(n) => n,
            other => panic!("Can't use {:?}, it isn't number", other),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Callable(c) => write!(f, "<callable {:x?}>", c),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EvalError(String);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", self.0)
    }
}

pub type EvalResult = Result<Value, EvalError>;

type Callable = fn(Vec<Value>) -> EvalResult;

pub fn eval(expr: ast::Expr) -> EvalResult {
    eval_with_env(expr, &mut make_global_env())
}

pub fn eval_with_env(expr: ast::Expr, env: &mut HashMap<String, Value>) -> EvalResult {
    use ast::Expr::*;
    match expr {
        Symbol(_, s) => env.get(&s).cloned().ok_or_else(|| EvalError(format!("eval: Undefined symbol {}", s))),
        Number(_, n) => Ok(Value::Number(n)),
        If(_, _, cond, true_then, false_then, _) => {
            let expr = if eval_with_env(*cond, env)?.is_truthy() {
                true_then
            } else {
                false_then
            };
            Ok(eval_with_env(*expr, env)?)
        },
        Define(_, _, sym, value, _) => {
            let value = eval_with_env(*value, env)?;
            let sym = to_sym(sym)?;
            env.insert(sym, value.clone());
            Ok(value)
        },
        Call(_, sym, args, _) => {
            let sym = to_sym(sym)?;
            match env.get(&sym) {
                Some(Value::Callable(c)) => {
                    c(args
                        .into_iter()
                        .map(|expr| eval_with_env(expr, env))
                        .collect::<Result<Vec<_>, _>>()?
                    )
                },
                _ => Err(EvalError(format!("eval: Invalid function {}", sym))),
            }
        }
    }
}

fn to_sym(token: ast::Token) -> Result<String, EvalError> {
    match token.kind {
        ast::TokenKind::Symbol(s) => Ok(s),
        other => Err(EvalError(format!("Token '{:?}' is not symbol", other))),
    }
}

fn last_or_nil(values: Vec<Value>) -> Value {
    values.last().cloned().unwrap_or(Value::Nil)
}

pub fn make_global_env() -> HashMap<String, Value> {
    let mut env = HashMap::new();

    env.insert(
        S("print"),
        Value::Callable(|values| {
            for value in values.iter() {
                println!("{}", value);
            }
            Ok(last_or_nil(values))
        }),
    );

    env.insert(
        S("exit"),
        Value::Callable(|values| {
            let code = values.into_iter().last().unwrap_or(Value::Number(0));
            std::process::exit(code.into_num() as i32)
        })
    );

    env.insert(
        S("begin"),
        Value::Callable(|values|{
            Ok(last_or_nil(values))
        })
    );

    env.insert(
        S("+"),
        Value::Callable(|values| {
            Ok(Value::Number(values.iter().map(|n| n.into_num()).sum()))
        })
    );

    env.insert(
        S("*"),
        Value::Callable(|values| {
            Ok(Value::Number(values.iter().map(|n| n.into_num()).product()))
        })
    );

    env.insert(
        S("-"), 
        Value::Callable(|values| {
            Ok(Value::Number(
                if let Some((first, rest)) = values.split_first() {
                    let first = first.into_num();
                    if rest.is_empty() {
                        -first
                    } else {
                        rest.iter().fold(first, |n, m| n - m.into_num())
                    }
                } else {
                    0
                }
            ))
        })
    );

    env.insert(
        S("/"),
        Value::Callable(|values| {
                if let Some((first, rest)) = values.split_first() {
                    let first = first.into_num();
                    Ok(Value::Number(
                            if rest.is_empty() {
                                1 / first
                            } else {
                                rest.iter().fold(first, |n, m| n / m.into_num())
                            }
                    ))
                } else {
                    Err(EvalError(S("Wrong number of arguments: /, 0")))
                }
        })
    );

    env
}
