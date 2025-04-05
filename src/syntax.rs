pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub enum Expr {
    Var(String),
    Const(i32),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}

pub enum Statement {
    Assign(String, Expr),
    While(Expr, Box<Statement>),
    Seq(Box<Statement>, Box<Statement>),
    Print(Expr),
}
