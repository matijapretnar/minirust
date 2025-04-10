use std::collections::HashMap;
use std::fmt;

pub struct StackFrame {
    variables: HashMap<String, i32>,
}

impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x, v) in &self.variables {
            write!(f, "{x}: {v}\n")?;
        }
        Ok(())
    }
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
