use super::eval::*;
use std::collections::HashMap;
use big_s::S;

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

    env.insert(
        S("="),
        Value::Callable(|values| {
            let first = values.first().unwrap().into_num();
            Ok(
                if values.iter().any(|x| x.into_num() != first) {
                    Value::Nil
                } else {
                    Value::Number(1)
                }
            )
        }
    ));

    env.insert(
        S("<"),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(
                if let Some(v) = vs.to_tuples() {
                    if v.iter().filter(|(a, b)| !(a.into_num() < b.into_num())).collect::<Vec<_>>().is_empty() {
                        Value::Number(1)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            )
        }
    ));

    env.insert(
        S(">"),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(
                if let Some(v) = vs.to_tuples() {
                    if v.iter().filter(|(a, b)| !(a.into_num() > b.into_num())).collect::<Vec<_>>().is_empty() {
                        Value::Number(1)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            )
        }
    ));

    env.insert(
        S("<="),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(
                if let Some(v) = vs.to_tuples() {
                    if v.iter().filter(|(a, b)| !(a.into_num() <= b.into_num())).collect::<Vec<_>>().is_empty() {
                        Value::Number(1)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            )
        }
    ));
    
    env.insert(
        S(">="),
        Value::Callable(|values| {
            let vs = Values::from(values);
            Ok(
                if let Some(v) = vs.to_tuples() {
                    if v.iter().filter(|(a, b)| !(a.into_num() >= b.into_num())).collect::<Vec<_>>().is_empty() {
                        Value::Number(1)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            )
        }
    ));

    env.insert(
        S("t"), 
        Value::Number(1)
    );

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
                    Some(n) => ret.push((*v, *n)),
                    None => break
                }
            }
            Some(ret)
        }
    }
}
