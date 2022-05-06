use crate::{frame::Frame, instr::Instr, label::Label, stack::Stack, val::Value};
use anyhow::{Result, Context};
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

/// Resolves a constant initializer expression to a runtime value
pub fn resolve_constant_expr(expr: &InitExpr) -> Result<Value> {
    let mut ops_reader = expr.get_operators_reader();
    let op = ops_reader.read()?;
    let instr = Instr::try_from(op)?;
    instr.const_value().with_context(|| format!("{:?} is not a constant instruction", instr))
}
