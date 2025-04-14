use minirust::{Statement, Expr, BinOp};

fn main() {
    let mut state = minirust::State::new();
    let stmt = minirust::Statement::fibonacci(10);
    state.functions.insert(
        "gcd".to_string(),
        (
            vec!["m".to_string(), "n".to_string()],
            Statement::ret(Expr::if_then_else(
                Expr::var("n"),
                Expr::call(
                    "gcd",
                    vec![
                        Expr::var("n"),
                        Expr::bin_op(BinOp::Mod, Expr::var("m"), Expr::var("n")),
                    ],
                ),
                Expr::var("m")
            )),
        ),
    );
    println!("{stmt}");
    stmt.run(&mut state);
    for msg in state.output() {
        println!("{msg}");
    }
    println!("{state}");
}
