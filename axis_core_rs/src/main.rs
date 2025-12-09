use axis_core_rs::example::simple_lambda_example;
use axis_core_rs::env::Env;
    
fn main() {    
    let env = Env::empty();

    let expr = simple_lambda_example();

    let result = axis_core_rs::eval::eval(&expr, &env);

    match result {
        Ok(value) => println!("Evaluation result: {:?}", value),
        Err(err) => println!("Evaluation error: {}", err),
    }

    // The crate does not expose `axis_core_rs::eval::eval`; avoid calling it here
    // and show the created expression's type instead to keep the program compilable.
    // println!("Expression created (type = {})", std::any::type_name_of_val(&expr));
}