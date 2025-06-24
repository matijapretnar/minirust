use std::fmt;

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Expr {
    Var(String),
    Constant(i32),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[macro_export]
macro_rules! expr {
    // MiniRust variable
    ($var:ident) => {
        $crate::Expr::Var(stringify!($var).to_string())
    };

    // Interpolated Rust variable
    ({ $var:ident }) => {
        $crate::Expr::Constant($var)
    };

    // Integer literal
    ($n:literal) => {
        $crate::Expr::Constant($n)
    };
    (($($inner:tt)*)) => { expr!($($inner)*) };

    // Binary operations, lowest precedence first
    ($left:tt + $($rest:tt)+) => {
        $crate::Expr::BinOp(
            $crate::BinOp::Add,
            Box::new(expr!($left)),
            Box::new(expr!($($rest)+))
        )
    };
    ($left:tt - $($rest:tt)+) => {
        $crate::Expr::BinOp(
            $crate::BinOp::Sub,
            Box::new(expr!($left)),
            Box::new(expr!($($rest)+))
        )
    };
    ($left:tt * $($rest:tt)+) => {
        $crate::Expr::BinOp(
            $crate::BinOp::Mul,
            Box::new(expr!($left)),
            Box::new(expr!($($rest)+))
        )
    };
    ($left:tt / $($rest:tt)+) => {
        $crate::Expr::BinOp(
            $crate::BinOp::Div,
            Box::new(expr!($left)),
            Box::new(expr!($($rest)+))
        )
    };
    ($left:tt % $($rest:tt)+) => {
        $crate::Expr::BinOp(
            $crate::BinOp::Mod,
            Box::new(expr!($left)),
            Box::new(expr!($($rest)+))
        )
    };
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
            Self::Call(fun, _exprs) => write!(f, "{fun}(...)"),
        }
    }
}
impl Expr {
    pub fn if_then_else(expr: Expr, expr1: Expr, expr2: Expr) -> Self {
        Self::IfThenElse(Box::new(expr), Box::new(expr1), Box::new(expr2))
    }

    pub fn call(fun: &str, exprs: Vec<Expr>) -> Self {
        Self::Call(String::from(fun), exprs)
    }
}

#[derive(Clone)]
pub enum Statement {
    Assign(String, Expr),
    DoWhile(Expr, Box<Statement>),
    Seq(Box<Statement>, Box<Statement>),
    Print(Expr),
    Ret(Expr),
}

#[macro_export]
macro_rules! statement {
    // Assignment: let x = expr
    (let $var:ident = $e:tt) => {
        $crate::Statement::Assign(stringify!($var).to_string(), expr!($e))
    };

    // Print: print expr
    (print $e:tt) => {
        $crate::Statement::Print(expr!($e))
    };

    // Return: return expr
    (return $e:tt) => {
        $crate::Statement::Ret(expr!($e))
    };
}

impl Statement {
    pub fn do_while(expr: Expr, stmt: Statement) -> Self {
        Self::DoWhile(expr, Box::new(stmt))
    }
    pub fn seq(stmt1: Statement, stmt2: Statement) -> Self {
        Self::Seq(Box::new(stmt1), Box::new(stmt2))
    }
    pub fn fibonacci(n: i32) -> Self {
        Statement::seq(
            statement!(let a = 0),
            Statement::seq(
                statement!(let b = 1),
                Statement::seq(
                    statement!(let i = 0),
                    Statement::do_while(
                        expr!(i + { n }),
                        Statement::seq(
                            statement!(print a),
                            Statement::seq(
                                statement!(let temp = a),
                                Statement::seq(
                                    statement!(let a = b),
                                    Statement::seq(
                                        statement!(let b = (temp + b)),
                                        statement!(let i = (i + 1)),
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

struct FormattedStatement<'a> {
    statement: &'a Statement,
    indent: u8,
}
impl FormattedStatement<'_> {
    fn write_indent(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 1..=self.indent {
            write!(f, " ")?;
        }
        Ok(())
    }
}
impl fmt::Display for FormattedStatement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.statement {
            Statement::Assign(x, expr) => {
                self.write_indent(f)?;
                write!(f, "LET {x} = {expr}")
            }
            Statement::DoWhile(expr, stmt) => {
                self.write_indent(f)?;
                write!(f, "WHILE {expr} {{\n")?;
                let fstmt = Self {
                    statement: stmt,
                    indent: self.indent + 2,
                };
                write!(f, "{fstmt}\n")?;
                self.write_indent(f)?;
                write!(f, "}}")?;
                Ok(())
            }
            Statement::Seq(stmt1, stmt2) => {
                let fstmt1 = Self {
                    statement: stmt1,
                    indent: self.indent,
                };
                let fstmt2 = Self {
                    statement: stmt2,
                    indent: self.indent,
                };
                write!(f, "{fstmt1};\n{fstmt2}")?;
                Ok(())
            }
            Statement::Print(expr) => {
                self.write_indent(f)?;
                write!(f, "PRINT {expr}")
            }
            Statement::Ret(expr) => {
                self.write_indent(f)?;
                write!(f, "RETURN {expr}")
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fstmt = FormattedStatement {
            statement: &self,
            indent: 0,
        };
        write!(f, "{fstmt}")
    }
}

pub struct Function {
    pub name: String,
    pub variables: Vec<String>,
    pub body: crate::Statement,
}

impl Function {
    pub fn gcd() -> Self {
        Self {
            name: "gcd".to_string(),
            variables: vec!["m".to_string(), "n".to_string()],
            body: Statement::seq(
                Statement::seq(statement!(print m), statement!(print n)),
                Statement::Ret(Expr::if_then_else(
                    expr!(n),
                    Expr::call("gcd", vec![expr!(n), expr!(m % n)]),
                    expr!(m),
                )),
            ),
        }
    }
}
