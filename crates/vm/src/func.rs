use crate::instance::Index as InstanceIndex;
use crate::instr::Instr;
use crate::val::Value;
use wasmparser::FuncType;

pub struct Func<'a> {
    ty: FuncType,
    instace_index: InstanceIndex,
    instructions: Vec<Instr<'a>>,
    locals: Vec<Value>,
}
