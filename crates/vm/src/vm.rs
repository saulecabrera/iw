use crate::{val::Value, frame::Frame, label::Label, stack::Stack};

pub struct VM {
    ip: usize,
    stack: Stack<StackEntry>,
}

pub enum StackEntry {
    Val(Value),
    CallFrame(Frame),
    Label(Label)
}

