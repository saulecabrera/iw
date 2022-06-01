use crate::{
    addressable::{Slot, Slottable},
    instr::Instr,
};
use anyhow::Result;
use wasmparser::{FuncType, LocalsReader, OperatorsReader, Type};

pub struct Func {
    ty: FuncType,
    locals: Vec<Type>,
    body: Vec<Instr>,
}

impl<'a> Func {
    pub fn new(ty: FuncType, locals: LocalsReader, body: OperatorsReader<'a>) -> Result<Self> {
        Ok(Self {
            ty,
            locals: locals
                .into_iter()
                .try_fold(Vec::new(), |mut acc, local| -> Result<_> {
                    let (count, ty) = local?;
                    // NB
                    // wasmparser returns the locals as (count, type) in order of appearance;
                    // here we "flatten" the locals into a vector of types
                    acc.extend_from_slice(&[ty].repeat(count as usize));
                    Ok(acc)
                })?,

            body: body
                .into_iter()
                .try_fold(Vec::new(), |mut acc, op| -> Result<_> {
                    acc.push(Instr::try_from(op?)?);
                    Ok(acc)
                })?,
        })
    }
}

impl Slottable for Func {
    fn slot() -> Slot {
        Slot::Func
    }
}
