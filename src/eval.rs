use crate::syntax::{BinOp, Expr, Statement};

pub struct Output {
    pub printouts: Vec<String>,
}

impl Output {
    pub fn new() -> Self {
        Output {
            printouts: Vec::new(),
        }
    }
    pub fn join(self, other: Self) -> Self {
        let mut printouts = self.printouts;
        printouts.extend(other.printouts);
        Output { printouts }
    }
    pub fn print(output: String) -> Self {
        Output {
            printouts: vec![output],
        }
    }
}

impl BinOp {
    pub fn eval(&self, m: i32, n: i32) -> i32 {
        match self {
            BinOp::Add => m + n,
            BinOp::Sub => m - n,
            BinOp::Mul => m * n,
            BinOp::Div => m / n,
            BinOp::Mod => m % n,
        }
    }
}

impl Expr {
    pub fn eval(&self, frame: &crate::state::StackFrame) -> i32 {
        match self {
            Expr::Var(x) => frame.read_variable(x),
            Expr::Constant(n) => *n,
            Expr::IfThenElse(expr, expr1, expr2) => {
                if expr.eval(frame) != 0 {
                    expr1.eval(frame)
                } else {
                    expr2.eval(frame)
                }
            }
            Expr::BinOp(op, expr1, expr2) => op.eval(expr1.eval(frame), expr2.eval(frame)),
        }
    }
}

impl Statement {
    fn pass() -> Output {
        Output::new()
    }
    fn run_two(stmt1: &Self, stmt2: &Self, frame: &mut crate::state::StackFrame, whole: &Statement) -> Output {
        let out1 = stmt1.run(frame, whole);
        // TODO: spremeni, če želiš "klikati" skozi izvajanje programa
        let out2 = stmt2.run(frame, whole);
        out1.join(out2)
    }
    pub fn run(&self, frame: &mut crate::state::StackFrame, whole: &Statement) -> Output {
        whole.print_active(self);
        println!("{frame}");
        match self {
            Statement::Assign(x, expr) => {
                let v = expr.eval(frame);
                frame.set_variable(x.clone(), v);
                Self::pass()
            }
            Statement::DoWhile(expr, stmt) => {
                let v = expr.eval(frame);
                if v != 0 {
                    Self::run_two(stmt, self, frame, whole)
                } else {
                    Self::pass()
                }
            }
            Statement::Seq(stmt1, stmt2) => Self::run_two(stmt1, stmt2, frame, whole),
            Statement::Print(expr) => {
                let v = expr.eval(frame);
                Output::print(format!("{v}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;

    fn new_frame(bindings: Vec<(&str, i32)>) -> state::StackFrame {
        let mut frame = state::StackFrame::new();
        for (x, v) in bindings {
            frame.set_variable(String::from(x), v);
        }
        frame
    }
    #[test]
    fn answer_expression() {
        let frame = new_frame(vec![("x", 7)]);
        let expr = Expr::BinOp(
            BinOp::Mul,
            Box::new(Expr::Constant(6)),
            Box::new(Expr::Var(String::from("x"))),
        );
        let result = expr.eval(&frame);
        assert_eq!(result, 42);
    }

    #[test]
    fn countdown_statement() {
        let mut frame = new_frame(vec![("x", 10)]);
        let stmt = Statement::DoWhile(
            Expr::Var(String::from("x")),
            Box::new(Statement::Seq(
                Box::new(Statement::Print(Expr::Var(String::from("x")))),
                Box::new(Statement::Assign(
                    String::from("x"),
                    Expr::BinOp(
                        BinOp::Sub,
                        Box::new(Expr::Var(String::from("x"))),
                        Box::new(Expr::Constant(1)),
                    ),
                )),
            )),
        );
        let output = stmt.run(&mut frame, &stmt);
        assert_eq!(
            output.printouts,
            vec!["10", "9", "8", "7", "6", "5", "4", "3", "2", "1"]
        )
    }
}
