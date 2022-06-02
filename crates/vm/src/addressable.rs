use crate::instance::Index as InstanceIndex;
use std::collections::HashMap;
use std::hash::Hash;

pub type ElemIndex = u32;

/// Represents the index space to which a particular
/// address refers to
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Slot {
    Func,
    Global,
    Table,
    Elem,
}

pub trait Slottable {
    fn slot() -> Slot;
}

/// Represents an address of an element instance in the Store.
/// An address is a hash key, made of the following elements:
///
/// * InstanceIndex: The module instance which owns particular element instances in the store
/// * ElemIndex: The location of a particular element in its index space
/// * Slot: The type of the index space
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Addr(InstanceIndex, ElemIndex, Slot);

impl Addr {
    pub fn new_unsafe(instance_index: InstanceIndex, element_index: ElemIndex, slot: Slot) -> Self {
        Self(instance_index, element_index, slot)
    }

    pub fn instance_index(&self) -> InstanceIndex {
        self.0
    }

    pub fn element_index(&self) -> ElemIndex {
        self.1
    }
}

pub struct Addressable<T: Slottable> {
    addresses: HashMap<Addr, T>,
}

impl<T: Slottable> Default for Addressable<T> {
    fn default() -> Self {
        Self {
            addresses: HashMap::<Addr, T>::new(),
        }
    }
}

impl<T: Slottable> Addressable<T> {
    pub fn push(&mut self, instance_index: InstanceIndex, elem_index: ElemIndex, val: T) -> Addr {
        let key = Addr::new_unsafe(instance_index, elem_index, T::slot());

        self.addresses.insert(key, val);

        key
    }

    pub fn get(&self, addr: &Addr) -> Option<&T> {
        self.addresses.get(&addr)
    }
}

#[cfg(test)]
mod tests {
    use super::{Addr, Addressable, Slot, Slottable};

    impl Slottable for usize {
        fn slot() -> Slot {
            Slot::Func
        }
    }

    impl Slottable for &str {
        fn slot() -> Slot {
            Slot::Func
        }
    }

    impl Slottable for String {
        fn slot() -> Slot {
            Slot::Func
        }
    }

    #[test]
    fn push_and_get() {
        let mut usizes: Addressable<usize> = Addressable::default();
        let mut strings: Addressable<String> = Addressable::default();

        let usize_addr = usizes.push(0, 0, 1);
        let usize_result = usizes.get(&usize_addr).unwrap();

        assert_eq!(usize_addr, Addr::new_unsafe(0, 0, Slot::Func));
        assert_eq!(*usize_result, 1usize);

        let string_addr = strings.push(1, 0, "foo".into());
        let string_result = strings.get(&string_addr).unwrap();

        assert_eq!(string_addr, Addr::new_unsafe(1, 0, Slot::Func));
        assert_eq!(string_result, "foo");
    }
}
