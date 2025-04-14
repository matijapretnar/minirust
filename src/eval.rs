use crate::syntax::{BinOp, Expr, Statement};

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
    pub fn eval(&self, state: &crate::state::State) -> i32 {
        match self {
            Expr::Var(x) => state.read_variable(x),
            Expr::Constant(n) => *n,
            Expr::IfThenElse(expr, expr1, expr2) => {
                if expr.eval(state) != 0 {
                    expr1.eval(state)
                } else {
                    expr2.eval(state)
                }
            }
            Expr::BinOp(op, expr1, expr2) => op.eval(expr1.eval(state), expr2.eval(state)),
        }
    }
}

impl Statement {
    pub fn run(&self, state: &mut crate::state::State) {
        match self {
            Statement::Assign(x, expr) => {
                let v = expr.eval(state);
                state.set_variable(x.clone(), v)
            }
            Statement::DoWhile(expr, stmt) => {
                let v = expr.eval(state);
                if v != 0 {
                    stmt.run(state);
                    self.run(state)
                }
            }
            Statement::Seq(stmt1, stmt2) => {
                stmt1.run(state);
                stmt2.run(state)
            }
            Statement::Print(expr) => {
                let v = expr.eval(state);
                state.print(v.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StackFrame, State};

    #[test]
    fn answer_expression() {
        let state = State {
            frame: StackFrame::from_bindings(vec![("x", 7)]),
            changes: Vec::new(),
        };
        let expr = Expr::BinOp(
            BinOp::Mul,
            Box::new(Expr::Constant(6)),
            Box::new(Expr::Var(String::from("x"))),
        );
        let result = expr.eval(&state);
        assert_eq!(result, 42);
    }

    #[test]
    fn countdown_statement() {
        let mut state = State {
            frame: StackFrame::from_bindings(vec![("x", 10)]),
            changes: Vec::new(),
        };
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
        stmt.run(&mut state);
        assert_eq!(
            state.output(),
            vec!["10", "9", "8", "7", "6", "5", "4", "3", "2", "1"]
        )
    }
}
