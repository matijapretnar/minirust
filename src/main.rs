use minirust::{expr, Expr, Function, Statement};

fn main() {
    let mut state = minirust::State::new();
    let stmt = Statement::Print(Expr::call("gcd", vec![expr!(42), expr!(15)]));
    state.add_function(Function::gcd());
    println!("{stmt}");
    stmt.run(&mut state);
    for msg in state.output() {
        println!("{msg}");
    }
    println!("{state}");
}
