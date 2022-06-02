use crate::{
    addressable::{Addr, Slot, Slottable},
    frame::Frame,
    instance::{Func, Index},
    instr::Instr,
    label::Label,
    stack::Stack,
    val::{RefValue, Value},
};
use anyhow::{Context, Result};
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
    instr
        .const_value()
        .with_context(|| format!("{:?} is not a constant instruction", instr))
}

/// Resolves a funcref initializer expr
pub fn resolve_funcref_expr(expr: &InitExpr, instance_index: Index) -> Result<RefValue> {
    let mut ops_reader = expr.get_operators_reader();
    let op = ops_reader.read()?;
    let instr = Instr::try_from(op)?;
    let func_idx = instr
        .funcref_idx()
        .with_context(|| format!("{:?} is not a function reference", instr))?;

    let addr = Addr::new_unsafe(instance_index, func_idx, Func::slot());

    Ok(RefValue::FuncRef(addr))
}
