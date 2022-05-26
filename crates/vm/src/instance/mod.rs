pub mod elem;
pub mod func;
pub mod global;
pub mod table;
pub use func::*;
pub use global::*;
pub use table::*;

use wasmparser::FuncType;

pub type Index = usize;
// TODO(@saulecabrera): Add imports once extern values are supported
#[derive(Debug, Clone)]
pub struct Instance {
    index: Index,
    types: Vec<FuncType>,
}

impl Instance {
    pub(crate) fn new(types: Vec<FuncType>, store_index: Index) -> Self {
        Instance {
            index: store_index,
            types,
        }
    }
}
