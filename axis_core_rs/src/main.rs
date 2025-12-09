use axis_core_rs::env::Env;
use axis_core_rs::eval;
use axis_core_rs::example::simple_lambda_example;
use axis_core_rs::Expr;

use std::sync::Arc;

fn main() {
    let env = Env::empty();
    let exprs: Vec<Expr> = (0..100_000)
        .map(|i| Expr::Int(i)) // trivial expression for now
        .collect();

    let start = std::time::Instant::now();
    let _results = axis_core_rs::parallel::eval_many_parallel(&exprs, &env);
    let dur_par = start.elapsed();

    let start_seq = std::time::Instant::now();
    let _results_seq: Vec<_> = exprs.iter().map(|e| eval(e, &env)).collect();
    let dur_seq = start_seq.elapsed();

    println!("Sequential: {:?}", dur_seq);
    println!("Parallel: {:?}", dur_par);
}
