use anyhow::Result;
use crate::val::ValueType;
use wasmparser::Type;

pub struct Table {
    ty: ValueType,
    initial: u32,
    maximum: Option<u32>,
}

impl Table {
    pub fn new(ty: Type, initial: u32, maximum: Option<u32>) -> Result<Self> {
        unimplemented!()
    }
}
