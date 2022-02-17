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
    pub fn push(&mut self, module_index: ModuleIndex, val: T) -> Addr<T> {
        let index = self.addresses.len();
        self.addresses
            .insert(Addr::<T>::new_unsafe(module_index, index), val);

        Addr::<T>::new_unsafe(module_index, index)
    }

    pub fn get(&self, addr: &Addr<T>) -> Option<&T> {
        self.addresses.get(&addr)
    }
}

#[cfg(test)]
mod tests {
    use super::{Addr, Addressable};

    #[test]
    fn push_and_get() {
        let mut usizes: Addressable<usize> = Addressable::default();
        let mut strings: Addressable<String> = Addressable::default();

        let usize_addr = usizes.push(0, 1);
        let usize_result = usizes.get(&usize_addr).unwrap();

        assert_eq!(usize_addr, Addr::new_unsafe(0, 0));
        assert_eq!(*usize_result, 1usize);

        let string_addr = strings.push(0, "foo".into());
        let string_result = strings.get(&string_addr).unwrap();

        assert_eq!(string_addr, Addr::new_unsafe(0, 0));
        assert_eq!(string_result, "foo");
    }
}
