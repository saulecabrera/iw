use crate::val::ValueType;
use anyhow::Result;

pub struct Table {
    ty: ValueType,
    initial: u32,
    maximum: Option<u32>,
}

impl Table {
    pub fn new(ty: ValueType, initial: u32, maximum: Option<u32>) -> Result<Self> {
        Ok(Self {
            ty,
            initial,
            maximum,
        })
    }
}
