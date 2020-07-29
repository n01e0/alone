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

    env
}

fn last_or_nil(values: Vec<Value>) -> Value {
    values.last().cloned().unwrap_or(Value::Nil)
}
