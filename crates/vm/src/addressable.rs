use crate::instance::Index as InstanceIndex;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Addr(InstanceIndex);

impl Addr {
    pub fn new_unsafe(instance_index: InstanceIndex) -> Self {
        Self(instance_index)
    }

    pub fn instance_index(&self) -> InstanceIndex {
        self.0
    }
}

pub struct Addressable<T> {
    addresses: HashMap<Addr, T>,
}

impl<T> Default for Addressable<T> {
    fn default() -> Self {
        Self {
            addresses: HashMap::<Addr, T>::new(),
        }
    }
}

// 1. Instance index: is the instance that we are currently allocating
// 2. T index, which represents the where in the index space T appears
// 3. T the actual instance
//
// ---
//
// key: <instance_index, index> = guarantee order on index
impl<T> Addressable<T> {
    pub fn push(&mut self, instance_index: InstanceIndex, val: T) -> Addr {
        let key = Addr::new_unsafe(instance_index);

        self.addresses.insert(key, val);

        key
    }

    pub fn get(&self, addr: &Addr) -> Option<&T> {
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

        assert_eq!(usize_addr, Addr::new_unsafe(0));
        assert_eq!(*usize_result, 1usize);

        let string_addr = strings.push(0, "foo".into());
        let string_result = strings.get(&string_addr).unwrap();

        assert_eq!(string_addr, Addr::new_unsafe(0));
        assert_eq!(string_result, "foo");
    }
}
