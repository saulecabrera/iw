use crate::module::Index as ModuleIndex;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Addr<T>(ModuleIndex, usize, PhantomData<T>);

impl<T> Addr<T> {
    pub fn new_unsafe(module_index: ModuleIndex, index: usize) -> Self {
        Self(module_index, index, PhantomData)
    }

    pub fn module_index(&self) -> ModuleIndex {
        self.0
    }
}

pub struct Addressable<T> {
    addresses: HashMap<Addr<T>, T>,
}

impl<T> Default for Addressable<T> {
    fn default() -> Self {
        Self {
            addresses: HashMap::<Addr<T>, T>::new(),
        }
    }
}

impl<T> Addressable<T>
where
    Addr<T>: Eq + Hash,
{
    pub fn push(&mut self, module_index: ModuleIndex, val: T) {
        let index = self.addresses.len();
        let addr = Addr::<T>::new_unsafe(module_index, index);
        self.addresses.insert(addr, val);
    }
}
