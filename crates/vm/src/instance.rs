use std::borrow::BorrowMut;

use wasmparser::{FuncType, Global};
use crate::module::Module;
use crate::store::Store;
use anyhow::Result;

pub type Index = usize;
// TODO(@saulecabrera): Add imports once extern values are supported
#[derive(Debug, Clone)]
pub struct Instance {
    index: Index,
    types: Vec<FuncType>,
}

impl Instance {
    pub(crate) fn new(store: &mut Store, module: &Module, store_index: Index) -> Result<Self> {
        let types = module.func_types();

        Self::allocate(store.borrow_mut(), module)?;

        Ok(Instance {
            index: store_index,
            types,
        })
    }

    fn allocate(store: &mut Store, module: &Module) -> Result<()> {
        Self::allocate_globals(store.borrow_mut(), &module.globals)
    }

    fn allocate_globals(store: &mut Store, globals: &Vec<Global>) -> Result<()> {
        globals.iter().for_each(|global| {

        });

        Ok(())
    }
}
