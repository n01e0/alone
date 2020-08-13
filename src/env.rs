use super::eval::*;
use big_s::S;
use itertools::Itertools;
use std::collections::HashMap;

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
        }),
    );

    env.insert(
        S("begin"),
        Value::Callable(|values| Ok(last_or_nil(values))),
    );

    env.insert(
        S("+"),
        Value::Callable(|values| {
            Ok(Value::Number(
                values.iter().map(|n| n.clone().into_num()).sum(),
            ))
        }),
    );

    env.insert(
        S("*"),
        Value::Callable(|values| {
            Ok(Value::Number(
                values.iter().map(|n| n.clone().into_num()).product(),
            ))
        }),
    );

    env.insert(
        S("-"),
        Value::Callable(|values| {
            Ok(Value::Number(
                if let Some((first, rest)) = values.split_first() {
                    let first = first.clone().into_num();
                    if rest.is_empty() {
                        -first
                    } else {
                        rest.iter().fold(first, |n, m| n - m.clone().into_num())
                    }
                } else {
                    0
                },
            ))
        }),
    );

    env.insert(
        S("/"),
        Value::Callable(|values| {
            if let Some((first, rest)) = values.split_first() {
                let first = first.clone().into_num();
                Ok(Value::Number(if rest.is_empty() {
                    1 / first
                } else {
                    rest.iter().fold(first, |n, m| n / m.clone().into_num())
                }))
            } else {
                Err(EvalError(S("Wrong number of arguments: /, 0")))
            }
        }),
    );

    env.insert(
        S("="),
        Value::Callable(|values| {
            let first = values.first().unwrap().clone().into_num();
            Ok(if values.iter().any(|x| x.clone().into_num() != first) {
                Value::Nil
            } else {
                Value::Number(1)
            })
        }),
    );

    env.insert(S("eq"), env["="].clone());

    env.insert(
        S("!"),
        Value::Callable(|values| match values.len() {
            1 => Ok(if values.first().unwrap().is_truthy() {
                Value::Nil
            } else {
                Value::Number(1)
            }),
            n if n > 1 => Err(EvalError(S("too many arguments given to NOT"))),
            _ => Err(EvalError(S("too few arguments givien to NOT"))),
        }),
    );

    env.insert(S("not"), env["!"].clone());

    env.insert(
        S("<"),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(if let Some(v) = vs.to_tuples() {
                if v.iter()
                    .filter(|(a, b)| !(a.clone().into_num() < b.clone().into_num()))
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    Value::Number(1)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            })
        }),
    );

    env.insert(
        S(">"),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(if let Some(v) = vs.to_tuples() {
                if v.iter()
                    .filter(|(a, b)| !(a.clone().into_num() > b.clone().into_num()))
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    Value::Number(1)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            })
        }),
    );

    env.insert(
        S("<="),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(if let Some(v) = vs.to_tuples() {
                if v.iter()
                    .filter(|(a, b)| !(a.clone().into_num() <= b.clone().into_num()))
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    Value::Number(1)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            })
        }),
    );

    env.insert(
        S(">="),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(if let Some(v) = vs.to_tuples() {
                if v.iter()
                    .filter(|(a, b)| !(a.clone().into_num() >= b.clone().into_num()))
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    Value::Number(1)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            })
        }),
    );

    env.insert(
        S("cons"),
        Value::Callable(|values| {
            if let Some((a, b)) = values.iter().next_tuple() {
                Ok(Value::Cons(Cons::new(a.clone(), b.clone())))
            } else {
                Ok(Value::Nil)
            }
        }),
    );

    env.insert(
        S("list"),
        Value::Callable(|values| {
            if values.len() > 1 {
                let (first, rest) = values.split_first().unwrap();
                let mut ret = Cons::new(first.clone(), Value::Nil);
                for v in rest {
                    ret.append(v.clone());
                }
                Ok(Value::Cons(ret))
            } else {
                Ok(Value::Cons(Cons::new(
                    values.first().unwrap().clone(),
                    Value::Nil,
                )))
            }
        }),
    );

    env.insert(
        S("car"),
        Value::Callable(|values| match values.first() {
            Some(Value::Cons(cons)) => Ok(cons.clone().car()),
            _ => Err(EvalError(S("Wrong argument type: car require cons"))),
        }),
    );

    env.insert(
        S("cdr"),
        Value::Callable(|values| match values.first() {
            Some(Value::Cons(cons)) => Ok(cons.clone().cdr()),
            _ => Err(EvalError(S("Wrong argument type: car require cons"))),
        }),
    );

    env.insert(S("T"), Value::Number(1));

    env.insert(S("t"), Value::Number(1));

    env
}

fn last_or_nil(values: Vec<Value>) -> Value {
    values.last().cloned().unwrap_or(Value::Nil)
}

struct Values(Vec<Value>);
impl Values {
    pub fn from(v: Vec<Value>) -> Self {
        Self(v)
    }

    pub fn to_tuples(&self) -> Option<Vec<(Value, Value)>> {
        if self.0.len() < 2 {
            None
        } else {
            let mut ret = Vec::new();
            for (i, v) in self.0.iter().enumerate() {
                match self.0.iter().nth(i + 1) {
                    Some(n) => ret.push((v.clone(), n.clone())),
                    None => break,
                }
            }
            Some(ret)
        }
    }
}
