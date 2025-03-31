mod state;
mod syntax;

fn main() {
    use syntax::{BinOp, Expr, Statement};
    let mut frame = state::StackFrame::new();
    frame.set_variable(String::from("x"), 7);
    let expr = Expr::BinOp(
        BinOp::Mul,
        Box::new(Expr::Const(6)),
        Box::new(Expr::Var(String::from("x"))),
    );
    let result = expr.eval(&frame);
    // TODO: Mar se da to preverjati malo bolj samodejno?
    if result == 42 {
        println!("Koda dela pravilno.");
    } else {
        println!("Koda ne dela pravilno.");
    }
    let stmt = Statement::While(
        Expr::Var(String::from("x")),
        Box::new(Statement::Seq(
            Box::new(Statement::Print(Expr::Var(String::from("x")))),
            Box::new(Statement::Assign(
                String::from("x"),
                Expr::BinOp(
                    BinOp::Sub,
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Const(1)),
                ),
            )),
        )),
    );
    stmt.run(&mut frame);
}
