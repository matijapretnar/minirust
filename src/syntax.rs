use std::fmt;

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
        }
    }
}

pub enum Expr {
    Var(String),
    Constant(i32),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(x) => write!(f, "{x}"),
            Self::Constant(n) => write!(f, "{n}"),
            Self::IfThenElse(expr, expr1, expr2) => {
                write!(f, "IF {expr} THEN {expr1} ELSE {expr2}")
            }
            Self::BinOp(op, expr1, expr2) => write!(f, "{expr1} {op} {expr2}"),
        }
    }
}
impl Expr {
    pub fn var(x: &str) -> Self {
        Self::Var(String::from(x))
    }
    pub fn constant(v: i32) -> Self {
        Self::Constant(v)
    }

    pub fn if_then_else(expr: Expr, expr1: Expr, expr2: Expr) -> Self {
        Self::IfThenElse(Box::new(expr), Box::new(expr1), Box::new(expr2))
    }

    pub fn bin_op(op: BinOp, expr1: Expr, expr2: Expr) -> Self {
        Self::BinOp(op, Box::new(expr1), Box::new(expr2))
    }
}

pub enum Statement {
    Assign(String, Expr),
    DoWhile(Expr, Box<Statement>),
    Seq(Box<Statement>, Box<Statement>),
    Print(Expr),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(x, expr) => write!(f, "LET {x} = {expr}"),
            Self::DoWhile(expr, stmt) => write!(f, "WHILE {expr} {{\n{stmt}\n}}"),
            Self::Seq(stmt1, stmt2) => write!(f, "{stmt1};\n{stmt2}"),
            Self::Print(expr) => write!(f, "PRINT {expr}"),
        }
    }
}
impl Statement {
    pub fn assign(x: &str, expr: Expr) -> Self {
        Self::Assign(String::from(x), expr)
    }
    pub fn do_while(expr: Expr, stmt: Statement) -> Self {
        Self::DoWhile(expr, Box::new(stmt))
    }
    pub fn seq(stmt1: Statement, stmt2: Statement) -> Self {
        Self::Seq(Box::new(stmt1), Box::new(stmt2))
    }
    pub fn print(expr: Expr) -> Self {
        Self::Print(expr)
    }
    pub fn fibonacci(n: i32) -> Self {
        Statement::seq(
            Statement::assign("a", Expr::constant(0)),
            Statement::seq(
                Statement::assign("b", Expr::constant(1)),
                Statement::seq(
                    Statement::assign("i", Expr::constant(0)),
                    Statement::do_while(
                        Expr::bin_op(BinOp::Sub, Expr::var("i"), Expr::constant(n)),
                        Statement::seq(
                            Statement::print(Expr::var("a")),
                            Statement::seq(
                                Statement::assign("temp", Expr::var("a")),
                                Statement::seq(
                                    Statement::assign("a", Expr::var("b")),
                                    Statement::seq(
                                        Statement::assign(
                                            "b",
                                            Expr::bin_op(
                                                BinOp::Add,
                                                Expr::var("temp"),
                                                Expr::var("b"),
                                            ),
                                        ),
                                        Statement::assign(
                                            "i",
                                            Expr::bin_op(
                                                BinOp::Add,
                                                Expr::var("i"),
                                                Expr::constant(1),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )
    }
}
