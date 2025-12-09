use crate::{ast::Expr, env::Env, eval, value::Value};
use rayon::prelude::*;

/// Example: evaluate a list of expressions in parallel, sharing the same Env.
/// This is *semantically* safe because Env and Expr are immutable.
/// Later: we can build a proper parallel execution planner that walks the AST,
/// identifies independent subgraphs, and evaluates them via rayon::join / par_iter.
/// 
pub fn eval_many_parallel(exprs: &[Expr], env: &Env) -> Vec<Result<Value, eval::EvalError>> {
    exprs
        .par_iter()
        .map(|expr| eval(expr, env))
        .collect()
}

