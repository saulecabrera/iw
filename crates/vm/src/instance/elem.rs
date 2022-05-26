use crate::{
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
