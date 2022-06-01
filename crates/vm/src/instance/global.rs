use crate::{
    addressable::{Slot, Slottable},
    val::Value,
};
use wasmparser::GlobalType;

pub struct Global {
    val: Value,
    ty: GlobalType,
}

impl Global {
    pub fn new(val: Value, ty: GlobalType) -> Self {
        Self { val, ty }
    }
}

impl Slottable for Global {
    fn slot() -> Slot {
        Slot::Global
    }
}
