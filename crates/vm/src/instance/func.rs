use crate::instr::Instr;
use anyhow::Result;
use wasmparser::{FuncType, LocalsReader, OperatorsReader, Type};

pub struct Func<'a> {
    ty: FuncType,
    locals: Vec<(u32, Type)>,
    body: Vec<Instr<'a>>,
}

impl<'a> Func<'a> {
    pub fn new(ty: FuncType, locals: LocalsReader, body: OperatorsReader) -> Result<Self> {
        todo!()
    }
}
