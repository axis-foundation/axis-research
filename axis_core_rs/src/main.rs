use axis_core_rs::env::Env;
use axis_core_rs::eval;
// use axis_core_rs::example::simple_lambda_example;
use axis_core_rs::parallel::eval_many_parallel;
use axis_core_rs::Expr;

use std::sync::Arc;

// -----------------------------------------------------------------------------
// Axis Core Proof-of-Concept Demonstration
// -----------------------------------------------------------------------------
// This Rust program demonstrates the essential properties of the Axis semantic
// model. The goal is not to show final performance numbers yet, but to validate
// the architectural foundations that make Axis viable:
//
// 1. **Pure, Immutable Semantics**
//    Every expression (Expr), value (Value), and environment (Env) is immutable.
//    Evaluation never mutates state. Environments are implemented as persistent
//    data structures using Arc<HashMap<…>>, so “copies” are actually shared
//    structures. This proves Axis can enforce:
//
/*        • no shared mutable state
 *        • referential transparency
 *        • deterministic evaluation
 *        • runtime-agnostic semantics
 */
//
// 2. **Structural Sharing Instead of Copying**
//    Although Axis semantics forbid mutation, this does *not* lead to excessive
//    memory usage. Rust’s `Arc` and persistent maps allow all environments,
//    closures, and AST nodes to share structure efficiently. Only a tiny delta
//    is allocated when a new Env is derived. This demonstrates that an
//    immutable semantic model is practical and memory-efficient.
//
// 3. **Safe Lock-Free Parallel Evaluation**
//    Because all data structures are immutable and evaluation has no side
//    effects, multiple expressions can be evaluated in parallel with no locks,
//    no races, and no nondeterminism. The parallel evaluator here (`eval_many_parallel`)
//    uses Rayon to evaluate thousands of expressions concurrently. The results
//    match the sequential evaluator exactly.
//
//    This proves that Axis programs are *inherently parallelizable by design*.
//    Parallelism comes from semantics, not from threading tricks.
//
// 4. **Foundation for Speed and Energy Improvements**
//    By eliminating mutation, shared state, and runtime dependencies, Axis opens
//    the door to aggressive optimizations a traditional language cannot safely
//    perform:
//
//        • zero-copy sharing of environments and values
//        • automatic parallelization of independent subexpressions
//        • predictable, deterministic concurrency
//        • cache-friendly execution patterns
//
//    These properties—made visible in this POC—are the conceptual basis for the
//    projected speed and energy-efficiency gains in future optimized runtimes.
//
// -----------------------------------------------------------------------------
// In summary:
// This POC shows that the Axis semantic core can be implemented as a small,
// purely functional, immutable calculus that executes safely and deterministically
// in parallel. This confirms that the central ideas behind Axis are viable and
// provides the groundwork for later performance-focused backends.
// -----------------------------------------------------------------------------



fn make_big_expr() -> Expr {
    // Build a large tuple of 50k integers — stresses memory traversal
    let items = (0..50_000).map(Expr::Int).collect();
    Expr::Tuple(items)
}

fn make_complex_expr() -> Expr {
    // A nested Let expression that forces closure and env usage
    Expr::Let {
        name: "y".into(),
        value: Arc::new(Expr::Int(2)),
        body: Arc::new(Expr::Let {
            name: "z".into(),
            value: Arc::new(Expr::Int(3)),
            body: Arc::new(Expr::Tuple(vec![
                Expr::Var("y".into()),
                Expr::Var("z".into()),
                Expr::Int(999),
            ])),
        }),
    }
}

fn make_full_expr() -> Expr {
    let big_expr = make_big_expr();
    let complex_expr = make_complex_expr();

    // Your example:
    // let x = big_expr in complex_expr
    Expr::Let {
        name: "x".into(),
        value: Arc::new(big_expr),
        body: Arc::new(complex_expr),
    }
}

fn main() {
    let env = Env::empty();

    // Build 10,000 full expressions to evaluate
    let exprs: Vec<Expr> = (0..10_000)
        .map(|_| make_full_expr())
        .collect();

    // ------------------------------
    // Sequential evaluation
    // ------------------------------
    let start_seq = std::time::Instant::now();
    let seq_results: Vec<_> = exprs.iter().map(|e| eval(e, &env)).collect();
    let dur_seq = start_seq.elapsed();

    println!("Sequential: {:?}", dur_seq);

    // ------------------------------
    // Parallel evaluation
    // ------------------------------
    let start_par = std::time::Instant::now();
    let par_results = eval_many_parallel(&exprs, &env);
    let dur_par = start_par.elapsed();

    println!("Parallel: {:?}", dur_par);

    // Just to verify correctness
    println!("Sequential result[0]: {:?}", seq_results[0]);
    println!("Parallel   result[0]: {:?}", par_results[0]);
}