pub mod func;
pub mod global;
pub use func::*;
pub use global::*;

use crate::{module::Module, store::Store, vm};
use anyhow::Result;
use func::Func as FuncInstance;
use global::Global as GlobalInstance;
use std::borrow::BorrowMut;
use wasmparser::{FuncType, Global as GlobalReader};

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

        Self::allocate(store.borrow_mut(), module, store_index)?;

        Ok(Instance {
            index: store_index,
            types,
        })
    }

    fn allocate(store: &mut Store, module: &Module, index: Index) -> Result<()> {
        Self::allocate_globals(store.borrow_mut(), &module.globals, index)?;
        Self::allocate_funcs(store.borrow_mut(), &module, index)
    }

    fn allocate_globals(
        store: &mut Store,
        globals: &Vec<GlobalReader>,
        index: Index,
    ) -> Result<()> {
        globals.iter().try_for_each(|global| {
            let value = vm::eval_const_expr(&global.init_expr)?;
            store.allocate_global(index, GlobalInstance::new(value, global.ty))
        })
    }

    fn allocate_funcs(store: &mut Store, module: &Module, index: Index) -> Result<()> {
        let functions = &module.functions;
        let codes = &module.codes;
        let types = &module.func_types();

        codes
            .iter()
            .zip(functions)
            .try_for_each(|(body, func_index)| {
                let locals = body.get_locals_reader()?;
                let ops = body.get_operators_reader()?;
                let ty = types
                    .get(*func_index as usize)
                    .ok_or(anyhow::anyhow!("Invalid func index {}", func_index))?;

                store.allocate_func(index, FuncInstance::new(ty.clone(), locals, ops)?)
            })
    }
}
