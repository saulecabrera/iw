use crate::{frame::Frame, instr::Instr, label::Label, stack::Stack, val::Value};
use anyhow::Result;
use wasmparser::InitExpr;

pub struct VM {
    ip: usize,
    stack: Stack<StackEntry>,
}

pub enum StackEntry {
    Val(Value),
    CallFrame(Frame),
    Label(Label),
}

pub fn eval_const_expr(expr: &InitExpr) -> Result<Value> {
    let mut ops_reader = expr.get_operators_reader();
    let op = ops_reader.read()?;
    let instr = Instr::new(op);
    instr.eval_const()
}
