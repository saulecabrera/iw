use crate::val::Value;
use anyhow::Result;
use wasmparser::Operator;

pub struct Instr<'a> {
    kind: Operator<'a>,
}

// TODO: Track the offset if it's ever needed
impl<'a> Instr<'a> {
    pub fn new(kind: Operator<'a>) -> Self {
        Instr { kind }
    }

    pub fn eval_const(&self) -> Result<Value> {
        match self.kind {
            Operator::I32Const { value } => Ok(Value::I32(value)),
            Operator::I64Const { value } => Ok(Value::I64(value)),
            Operator::F32Const { value } => Ok(Value::F32(value.bits())),
            Operator::F64Const { value } => Ok(Value::F64(value.bits())),
            _ => anyhow::bail!("Unsupported const instruction {:?}", self.kind),
        }
    }
}
