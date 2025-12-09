pub mod ast;
pub mod value;
pub mod env;
pub mod eval;
pub mod parallel;
pub mod example;

pub use ast::Expr;
pub use value::Value;
pub use env::Env;
pub use eval::{eval, EvalError};
