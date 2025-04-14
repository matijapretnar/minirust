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
    pub fn from_bindings(bindings: Vec<(&str, i32)>) -> Self {
        let mut frame = Self::new();
        for (x, v) in bindings {
            frame.set_variable(String::from(x), v);
        }
        frame
    }
}

pub struct State {
    pub frame: StackFrame,
    pub output: Vec<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            frame: StackFrame::new(),
            output: Vec::new(),
        }
    }
    pub fn read_variable(&self, x: &String) -> i32 {
        self.frame.read_variable(x)
    }
    pub fn set_variable(&mut self, x: String, v: i32) {
        self.frame.set_variable(x, v);
    }
    pub fn print(&mut self, msg: String) {
        self.output.push(msg)
    }
}
