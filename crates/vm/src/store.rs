use anyhow::{Result, bail};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use crate::instance::{Index as InstanceIndex, Instance};
use crate::module::Module;

pub struct Store {
    instances: Vec<Instance>,
    instances_env: HashMap<String, InstanceIndex>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            instances: Vec::new(),
            instances_env: HashMap::new(),
        }
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
}
