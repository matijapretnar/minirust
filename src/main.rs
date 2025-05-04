use minirust::{Expr, Function, Statement};

fn main() {
    let mut state = minirust::State::new();
    let stmt = Statement::print(Expr::call(
        "gcd",
        vec![Expr::constant(42), Expr::constant(15)],
    ));
    state.add_function(Function::gcd());
    println!("{stmt}");
    stmt.run(&mut state);
    for msg in state.output() {
        println!("{msg}");
    }
    println!("{state}");
}
