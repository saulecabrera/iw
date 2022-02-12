use crate::{frame::Frame, label::Label, stack::Stack, val::Value};

pub struct VM {
    ip: usize,
    stack: Stack<StackEntry>,
}

pub enum StackEntry {
    Val(Value),
    CallFrame(Frame),
    Label(Label),
}
