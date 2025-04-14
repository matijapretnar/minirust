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
    pub fn from_bindings(bindings: Vec<(&String, i32)>) -> Self {
        let mut frame = Self::new();
        for (x, v) in bindings {
            frame.set_variable(x.clone(), v);
        }
        frame
    }
}

#[derive(Clone)]
pub struct Stack {
    pub frames: Vec<StackFrame>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { frames: vec![] }
    }
    pub fn read_variable(&self, x: &String) -> i32 {
        self.frames.first().unwrap().read_variable(x)
    }
    pub fn set_variable(&mut self, x: String, v: i32) {
        self.frames.first_mut().unwrap().set_variable(x, v);
    }
    pub fn push_frame(&mut self, frame: StackFrame) {
        self.frames.push(frame)
    }
    pub fn iter(&self) -> impl Iterator<Item = &StackFrame> {
        self.frames.iter()
    }
}

pub enum Change {
    Print(String),
    Stack(Stack),
}

pub struct State {
    pub stack: Stack,
    changes: Vec<Change>,
    pub functions: HashMap<String, (Vec<String>, crate::Statement)>,
}

impl State {
    pub fn new() -> Self {
        let mut stack = Stack::new();
        stack.push_frame(StackFrame::new());
        State {
            stack,
            changes: Vec::new(),
            functions: HashMap::new(),
        }
    }
    pub fn read_variable(&self, x: &String) -> i32 {
        self.stack.read_variable(x)
    }
    pub fn set_variable(&mut self, x: String, v: i32) {
        self.stack.set_variable(x, v);
        self.changes.push(Change::Stack(self.stack.clone()))
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
    pub fn prepare_function(&mut self, fun: &String, vs: Vec<i32>) -> crate::Statement {
        let (xs, stmt) = self.functions.get(fun).unwrap();
        let stmt = stmt.clone();
        let new_frame = crate::StackFrame::new();
        self.stack.push_frame(new_frame);
        for (x, v) in xs.iter().zip(vs) {
            self.stack.set_variable(String::from(x), v);
        }
        stmt
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.stack.frames.first().unwrap().fmt(f)
    }
}
