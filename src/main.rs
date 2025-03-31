mod state {
    use std::collections::HashMap;

    pub struct StackFrame {
        variables: HashMap<String, i32>,
    }

    impl StackFrame {
        pub fn new() -> Self {
            StackFrame {
                variables: HashMap::new(),
            }
        }
        pub fn read_variable(&self, x: &String) -> i32 {
            match self.variables.get(x) {
                Some(v) => *v,
                None => 0,
            }
        }
        pub fn set_variable(&mut self, x: String, v: i32) {
            self.variables.insert(x, v);
        }
    }
}

enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl BinOp {
    fn eval(&self, m: i32, n: i32) -> i32 {
        match self {
            BinOp::Add => m + n,
            BinOp::Sub => m - n,
            BinOp::Mul => m * n,
            BinOp::Div => m / n,
            BinOp::Mod => m % n,
        }
    }
}

enum Expr {
    Var(String),
    Const(i32),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self, frame: &state::StackFrame) -> i32 {
        match self {
            Expr::Var(x) => frame.read_variable(x),
            Expr::Const(n) => *n,
            Expr::If(expr, expr1, expr2) => {
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

enum Statement {
    Assign(String, Expr),
    While(Expr, Box<Statement>),
    Seq(Box<Statement>, Box<Statement>),
    Print(Expr),
}

impl Statement {
    fn run(&self, frame: &mut crate::state::StackFrame) {
        match self {
            Statement::Assign(x, expr) => {
                let v = expr.eval(frame);
                frame.set_variable(x.clone(), v);
            }
            Statement::While(expr, stmt) => {
                let v = expr.eval(frame);
                if v != 0 {
                    stmt.run(frame);
                    // TODO: spremeni, če želiš "klikati" skozi izvajanje programa
                    self.run(frame)
                }
            }
            Statement::Seq(stmt1, stmt2) => {
                stmt1.run(frame);
                // TODO: spremeni, če želiš "klikati" skozi izvajanje programa
                stmt2.run(frame)
            }
            Statement::Print(expr) => {
                let v = expr.eval(frame);
                // TODO: to bo treba spremeniti, ko bomo imeli vmesnik, ki ni konzola
                println!("{v}")
            }
        }
    }
}

fn main() {
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
