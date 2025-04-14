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
    fn run_two(stmt1: &Self, stmt2: &Self, state: &mut crate::state::State) -> Option<i32> {
        match stmt1.run(state) {
            None => stmt2.run(state),
            Some(v) => Some(v),
        }
    }
    pub fn run(&self, state: &mut crate::state::State) -> Option<i32> {
        match self {
            Statement::Assign(x, expr) => {
                let v = expr.eval(state);
                state.set_variable(x.clone(), v);
                None
            }
            Statement::DoWhile(expr, stmt) => {
                let v = expr.eval(state);
                if v != 0 {
                    Self::run_two(stmt, self, state)
                } else {
                    None
                }
            }
            Statement::Seq(stmt1, stmt2) => Self::run_two(stmt1, stmt2, state),
            Statement::Print(expr) => {
                let v = expr.eval(state);
                state.print(v.to_string());
                None
            }
            Statement::Ret(expr) => {
                let v: i32 = expr.eval(state);
                Some(v)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::State;

    #[test]
    fn answer_expression() {
        let mut state = State::new();
        state.set_variable(String::from("x"), 7);
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
        let mut state = State::new();
        state.set_variable(String::from("x"), 10);
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
