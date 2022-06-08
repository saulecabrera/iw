use crate::addressable::{Addr, Addressable, Slottable};
use crate::instance::{
    elem::Elem, func::Func, global::Global, table::Table, Index as InstanceIndex, Instance,
};
use crate::module::Module;
use crate::val::{RefType, RefValue, ValueType};
use crate::vm;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use wasmparser::{ElementItem, Global as GlobalReader};

#[derive(Default)]
pub struct Store {
    instances: Vec<Instance>,
    instances_env: HashMap<String, InstanceIndex>,
    globals: Addressable<Global>,
    funcs: Addressable<Func>,
    tables: Addressable<Table>,
    elems: Addressable<Elem>,
}

impl<'a> Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instantiate(&mut self, module: &'a Module, name: Option<String>) -> Result<Instance> {
        let index = self.instances.len();

        if let Some(name) = &name {
            if self.instances_env.contains_key(name) {
                bail!("Instance with name {} already exists", name);
            }
            self.instances_env.insert(name.to_owned(), index);
        }

        self.allocate(module, index)?;

        let instance = Instance::new(module.func_types(), index);
        self.instances.push(instance.clone());

        Ok(instance)
    }

    fn allocate(&mut self, module: &'a Module, index: InstanceIndex) -> Result<()> {
        self.allocate_globals(&module.globals, index)?;
        self.allocate_funcs(module, index)?;
        self.allocate_tables(module, index)?;
        self.allocate_elems(module, index)
    }

    fn allocate_globals(
        &mut self,
        globals: &Vec<GlobalReader>,
        index: InstanceIndex,
    ) -> Result<()> {
        globals.iter().enumerate().try_for_each(|(i, global)| {
            let value = vm::resolve_constant_expr(&global.init_expr)?;
            let elem_index = i
                .try_into()
                .with_context(|| format!("Conversion of {} to u32 failed", i))?;
            self.globals
                .push(index, elem_index, Global::new(value, global.ty));
            Ok(())
        })
    }

    fn allocate_funcs(&mut self, module: &'a Module, index: InstanceIndex) -> Result<()> {
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
                    .with_context(|| format!("Invalid function index {}", func_index))?;

                self.funcs
                    .push(index, *func_index, Func::new(ty.clone(), locals, ops)?);
                Ok(())
            })
    }

    fn allocate_tables(&mut self, module: &'a Module, index: InstanceIndex) -> Result<()> {
        let tables = &module.tables;
        tables.iter().enumerate().try_for_each(|(i, t)| {
            let ty = RefType::try_from(t.element_type)?;
            let elem_index = i
                .try_into()
                .with_context(|| format!("Conversion of {} to u32 failed", i))?;
            self.tables
                .push(index, elem_index, Table::new(ty, t.initial, t.maximum)?);

            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }

    fn allocate_elems(&mut self, module: &'a Module, index: InstanceIndex) -> Result<()> {
        let elements = &module.elements;
        elements
            .iter()
            .enumerate()
            .try_for_each(|(element_index, e)| {
                let ty = RefType::try_from(e.ty)?;
                let items_reader = e.items.get_items_reader()?;
                let acc: Vec<RefValue> = vec![];

                let acc = items_reader.into_iter().try_fold(acc, |mut acc, item| {
                    let rv = match item? {
                        ElementItem::Func(idx) => {
                            let func_addr = Addr::new_unsafe(index, idx, Func::slot());
                            RefValue::FuncRef(func_addr)
                        }
                        ElementItem::Expr(init) => vm::resolve_funcref_expr(&init, index)?,
                    };
                    acc.push(rv);

                    Ok::<Vec<RefValue>, anyhow::Error>(acc)
                })?;

                let elem_instance = Elem::new(ty, acc, &e.kind)?;

                if elem_instance.is_active() {
                    let (table_index, offset) = elem_instance.metadata().with_context(|| {
                        format!("No metadata found on element kind {:?}", elem_instance)
                    })?;
                    let table_addr = Addr::new_unsafe(index, table_index, Table::slot());
                    let table = self
                        .tables
                        .get_mut(&table_addr)
                        .with_context(|| format!("Invalid table address {:?}", table_addr))?;
                    table.init(&offset, elem_instance.data.clone())?;
                }

                self.elems
                    .push(index, u32::try_from(element_index)?, elem_instance);

                Ok::<(), anyhow::Error>(())
            })?;

        Ok(())
    }
}
