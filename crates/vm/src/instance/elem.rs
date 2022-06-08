use anyhow::Result;

use crate::{
    addressable::{Slot, Slottable},
    val::{RefType, RefValue, Value},
    vm,
};

use wasmparser::ElementKind;

#[derive(Debug)]
pub enum ElemKind {
    Passive,
    Active { index: u32, offset: Value },
    Declared,
}

#[derive(Debug)]
pub struct Elem {
    ty: RefType,
    pub data: Vec<RefValue>,
    kind: ElemKind,
}

impl Elem {
    pub fn new(ty: RefType, data: Vec<RefValue>, kind: &ElementKind) -> Result<Self> {
        let kind = match kind {
            ElementKind::Passive => ElemKind::Passive,
            ElementKind::Declared => ElemKind::Declared,
            ElementKind::Active {
                table_index: idx,
                init_expr: operator,
            } => {
                let val = vm::resolve_constant_expr(&operator)?;
                ElemKind::Active {
                    index: *idx,
                    offset: val,
                }
            }
        };

        Ok(Self { ty, data, kind })
    }

    /// Is this element segment active?
    pub fn is_active(&self) -> bool {
        match self.kind {
            ElemKind::Active { .. } => true,
            _ => false,
        }
    }

    /// Retrieve the metada associated with an active data segment
    pub fn metadata(&self) -> Option<(u32, &Value)> {
        match &self.kind {
            ElemKind::Active { index, offset } => Some((*index, offset)),
            _ => None,
        }
    }
}

impl Slottable for Elem {
    fn slot() -> Slot {
        Slot::Elem
    }
}
