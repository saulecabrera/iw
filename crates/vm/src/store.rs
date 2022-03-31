use crate::addressable::Addressable;
use crate::instance::Func;
use crate::instance::Global;
use crate::instance::{Index as InstanceIndex, Instance};
use crate::module::Module;
use anyhow::{bail, Result};
use std::borrow::BorrowMut;
use std::collections::HashMap;

#[derive(Default)]
pub struct Store<'a> {
    instances: Vec<Instance>,
    instances_env: HashMap<String, InstanceIndex>,
    globals: Addressable<Global>,
    funcs: Addressable<Func<'a>>,
}

impl<'a> Store<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instantiate(&mut self, module: &Module, name: Option<String>) -> Result<Instance> {
        let index = self.instances.len();

        if let Some(name) = &name {
            if self.instances_env.contains_key(name) {
                bail!("Instance with name {} already exists", name);
            }
            self.instances_env.insert(name.to_owned(), index);
        }

        let instance = Instance::new(self.borrow_mut(), module, index)?;
        self.instances.push(instance.clone());

        Ok(instance)
    }

    pub fn allocate_global(&mut self, instance_index: InstanceIndex, global: Global) -> Result<()> {
        self.globals.push(instance_index, global);
        Ok(())
    }

    pub fn allocate_func(&mut self, index: InstanceIndex, func: Func<'a>) -> Result<()> {
        self.funcs.push(index, func);
        Ok(())
    }
}
