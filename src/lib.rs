mod eval;
mod gui;
mod state;
mod syntax;

pub use state::{StackFrame, State};
pub use syntax::{BinOp, Expr, Function, Statement};
