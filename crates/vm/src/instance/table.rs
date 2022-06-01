use crate::{
    addressable::{Slot, Slottable},
    val::RefType,
};
use anyhow::Result;

pub struct Table {
    ty: RefType,
    initial: u32,
    maximum: Option<u32>,
}

impl Table {
    pub fn new(ty: RefType, initial: u32, maximum: Option<u32>) -> Result<Self> {
        Ok(Self {
            ty,
            initial,
            maximum,
        })
    }
}

impl Slottable for Table {
    fn slot() -> Slot {
        Slot::Table
    }
}
