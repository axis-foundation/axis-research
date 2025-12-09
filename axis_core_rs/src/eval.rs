use crate::ast::Expr;
use crate::env::Env;
use crate::value::Value;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("Unbound variable: {0}")]
    UnboundVar(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Non-function value in application")]
    NotAFunction,
}

pub fn eval(expr: &Expr, env: &Env) -> Result<Value, EvalError> {
    use Expr::*;

    match expr {
        Int(i) => Ok(Value::Int(*i)),
        Bool(b) => Ok(Value::Bool(*b)),

        Var(name) => env
            .lookup(name)
            .cloned()
            .ok_or_else(|| EvalError::UnboundVar(name.clone())),

        Lambda { param, body } => Ok(Value::Closure {
            param: param.clone(),
            body: Arc::clone(body),
            env: env.clone(),
        }),

        Apply { func, arg } => {
            let func_val = eval(func, env)?;
            let arg_val = eval(arg, env)?;
            match func_val {
                Value::Closure { param, body, env: closure_env } => {
                    let new_env = closure_env.extend(param, arg_val);
                    eval(&body, &new_env)
                }
                _ => Err(EvalError::NotAFunction),
            }
        }

        Let { name, value, body } => {
            let val = eval(value, env)?;
            let new_env = env.extend(name.clone(), val);
            eval(body, &new_env)
        }

        If { cond, then_branch, else_branch } => {
            let cond_val = eval(cond, env)?;
            match cond_val {
                Value::Bool(true) => eval(then_branch, env),
                Value::Bool(false) => eval(else_branch, env),
                _ => Err(EvalError::TypeError(
                    "if condition must be a boolean".to_string(),
                )),
            }
        }

        Tuple(items) => {
            let mut vals = Vec::with_capacity(items.len());
            for item in items {
                vals.push(eval(item, env)?);
            }
            Ok(Value::Tuple(vals))
        }

        Record(fields) => {
            let mut vals = Vec::with_capacity(fields.len());
            for (k, vexpr) in fields {
                vals.push((k.clone(), eval(vexpr, env)?));
            }
            Ok(Value::Record(vals))
        }
    }
}
