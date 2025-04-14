use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
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

pub enum Change {
    Print(String),
    Frame(StackFrame),
}

pub struct State {
    pub frame: StackFrame,
    pub changes: Vec<Change>,
}

impl State {
    pub fn new() -> Self {
        State {
            frame: StackFrame::new(),
            changes: Vec::new(),
        }
    }
    pub fn read_variable(&self, x: &String) -> i32 {
        self.frame.read_variable(x)
    }
    pub fn set_variable(&mut self, x: String, v: i32) {
        self.frame.set_variable(x, v);
        self.changes.push(Change::Frame(self.frame.clone()))
    }
    pub fn print(&mut self, msg: String) {
        self.changes.push(Change::Print(msg))
    }
    pub fn output(&self) -> Vec<String> {
        self.changes
            .iter()
            .filter_map(|change| match change {
                Change::Print(msg) => Some(msg.clone()),
                _ => None,
            })
            .collect()
    }
}
