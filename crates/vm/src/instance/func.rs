use crate::instr::Instr;
use anyhow::Result;
use wasmparser::{FuncType, LocalsReader, OperatorsReader, Type};

pub struct Func<'a> {
    ty: FuncType,
    locals: Vec<Type>,
    body: Vec<Instr<'a>>,
}

impl<'a> Func<'a> {
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
                    acc.push(Instr::new(op?));
                    Ok(acc)
                })?,
        })
    }
}
