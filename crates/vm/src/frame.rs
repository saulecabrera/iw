use crate::val::Value;

pub struct Frame {
    locals: Vec<Value>,
    // TODO: Replace with a definition
    // of Instruction Pointer if there's one
    ip: usize,
}
