use super::ast;
use super::env::make_global_env;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Cons(Box<Value>, Box<Value>);

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i64),
    Callable(Callable),
    Cons(Cons),
    Nil
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Number(n) => *n != 0,
            _ => true,
        }
    }

    pub fn into_num(self) -> i64 {
        match self {
            Value::Number(n) => n,
            Value::Nil => 0,
            other => panic!("Can't use {:?}, it isn't number", other),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Callable(c) => write!(f, "<callable {:x?}>", c),
            Value::Cons(c) => write!(f, "({}, {})", c.0, c.1),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EvalError(pub String);

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
