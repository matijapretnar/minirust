use minirust::{BinOp, Expr, Statement};

fn main() {
    let mut state = minirust::State::new();
    let stmt = Statement::print(Expr::call(
        "gcd",
        vec![Expr::constant(42), Expr::constant(15)],
    ));
    state.functions.insert(
        "gcd".to_string(),
        (
            vec!["m".to_string(), "n".to_string()],
            Statement::seq(
                Statement::seq(
                    Statement::print(Expr::var("m")),
                    Statement::print(Expr::var("n")),
                ),
                Statement::ret(Expr::if_then_else(
                    Expr::var("n"),
                    Expr::call(
                        "gcd",
                        vec![
                            Expr::var("n"),
                            Expr::bin_op(BinOp::Mod, Expr::var("m"), Expr::var("n")),
                        ],
                    ),
                    Expr::var("m"),
                )),
            ),
        ),
    );
    println!("{stmt}");
    stmt.run(&mut state);
    for msg in state.output() {
        println!("{msg}");
    }
    println!("{state}");
}
