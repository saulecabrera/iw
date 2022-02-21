pub type Index = usize;

// Modules and Instances
//
// - Module
//      - Validate a given module
//      - wasmparser -> Module
//
// - Instantiation
//      - Resolve the exports
//      - Provide the exports
//      - Create the instance
//          - Load all the elements into the store
//
//      NOTES:
//      - Instantiation should rely on a linker-like
//        structure and algorithm to perform name
//        resolution. Name resolution is performed
//        through definitions, which can be either of:
//        Module, HostFunc, Extern;
//

pub struct Instance {}
