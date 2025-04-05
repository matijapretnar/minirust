mod eval;
mod state;
mod syntax;

fn main() {
    use syntax::{BinOp, Expr, Statement};
    let mut frame = state::StackFrame::new();
    frame.set_variable(String::from("x"), 10);
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
