use crate::addressable::Addressable;
use crate::instance::{func::Func, global::Global, table::Table, Index as InstanceIndex, Instance};
use crate::module::Module;
use crate::vm;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use wasmparser::Global as GlobalReader;

#[derive(Default)]
pub struct Store {
    instances: Vec<Instance>,
    instances_env: HashMap<String, InstanceIndex>,
    globals: Addressable<Global>,
    funcs: Addressable<Func>,
    tables: Addressable<Table>,
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
        self.allocate_tables(module, index)
    }

    fn allocate_globals(
        &mut self,
        globals: &Vec<GlobalReader>,
        index: InstanceIndex,
    ) -> Result<()> {
        globals.iter().try_for_each(|global| {
            let value = vm::resolve_constant_expr(&global.init_expr)?;
            self.globals.push(index, Global::new(value, global.ty));
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

                self.funcs.push(index, Func::new(ty.clone(), locals, ops)?);
                Ok(())
            })
    }

    fn allocate_tables(&mut self, module: &'a Module, index: InstanceIndex) -> Result<()> {
        unimplemented!()
    }
}
