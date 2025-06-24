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
    // Function calls
    ($func:ident ( $($args:tt),* )) => {
        $crate::Expr::Call(
            stringify!($func).to_string(),
            vec![$(expr!($args)),*]
        )
    };

    // MiniRust variable
    ($var:ident) => {
        $crate::Expr::Var(stringify!($var).to_string())
    };

    // Interpolated Rust variable
    ({ $expr:expr }) => { $expr };

    // Integer literal
    ($n:literal) => {
        $crate::Expr::Constant($n)
    };

    // If-then-else: if cond { then_branch } else { else_branch }
    (if $cond:tt $then_branch:tt else $else_branch:tt) => {
        $crate::Expr::IfThenElse(
            Box::new(expr!($cond)),
            Box::new(expr!($then_branch)),
            Box::new(expr!($else_branch))
        )
    };

    // Parentheses
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
    // Sequence of statements separated by ;
    (($($inner:tt)*)) => { statement!($($inner)*) };
    ({$($inner:tt)*}) => { statement!($($inner)*) };

    ($first:tt ; $($rest:tt)+) => {
        $crate::Statement::Seq(
            Box::new(statement!($first)),
            Box::new(statement!($($rest)+))
        )
    };

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

    // DoWhile: while expr { body }
    (while $cond:tt $body:tt) => {
        $crate::Statement::DoWhile(
            expr!($cond),
            Box::new(statement!($body))
        )
    };
}

impl Statement {
    pub fn fibonacci(n: i32) -> Self {
        statement!(
            (let a = 0);
            (let b = 1);
            (let i = 0);
            while (i + { Expr::Constant(n) }) {
                (print a) ;
                (let temp = a) ;
                (let a = b) ;
                (let b = (temp + b)) ;
                (let i = (i + 1))
            }
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
            body: statement! {
                (print m);
                (print n);
                (return (if n (gcd(n, (m % n))) else m))
            },
        }
    }
}
