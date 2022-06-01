use crate::{
    addressable::{Slot, Slottable},
    instr::Instr,
    val::{RefType, RefValue},
};

pub enum ElemKind {
    Passive,
    Active { table_index: u32, init_expr: Instr },
    Declared,
}

pub struct Elem {
    ty: RefType,
    refs: Vec<RefValue>,
    kind: ElemKind,
}

impl Elem {
    fn new(ty: RefType, refs: Vec<RefValue>, kind: ElemKind) -> Self {
        Self { ty, refs, kind }
    }
}

impl Slottable for Elem {
    fn slot() -> Slot {
        Slot::Elem
    }
}
