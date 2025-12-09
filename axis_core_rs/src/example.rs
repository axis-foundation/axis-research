use crate::ast::Expr;
use std::sync::Arc;

/// let inc = fn x => x + 1 in inc 41
///
/// (we don't have built-in + yet; we can add primitives later.
///  For now, this is more about structure.)
/// 
/// 
/// placeholder: later we add primitive ops as built-ins via a small runtime table.
/// 
pub fn simple_lambda_example() -> Expr {
    use Expr::*;

    let inc = Lambda {
        param: "x".to_string(),
        body: Arc::new(Var("x".to_string())), // placeholder until primitives
    };

    let body = Apply {
        func: Arc::new(Var("inc".to_string())),
        arg: Arc::new(Int(41)),
    };

    Let {
        name: "inc".to_string(),
        value: Arc::new(inc),
        body: Arc::new(body),
    }
}

