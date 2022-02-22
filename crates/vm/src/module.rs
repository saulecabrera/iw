use anyhow::Result;
use std::path::Path;
use wasmparser::*;

type Index = u32;

struct CustomSection<'a> {
    name: &'a str,
    data_offset: usize,
    data: &'a [u8],
    range: Range,
}

pub struct Module<'a> {
    // Sections not supported, yet
    // - Alias
    // - Instance
    // - Tag
    // - Module (`ModuleSectionStart`, `ModuleSectionEntry`)
    //
    // The custom name section can be used to obtain
    // function names. `NameSectionReader` can be used
    // to retrieve function names and `ExportSectionReader`
    // can be used to retrieve export names.
    // Ref: https://github.com/bytecodealliance/wasm-tools/issues/104
    //
    start_fn_idx: Option<Index>,

    types: Vec<TypeDef<'a>>,
    imports: Vec<Import<'a>>,
    functions: Vec<Index>,
    tables: Vec<TableType>,
    memories: Vec<MemoryType>,
    globals: Vec<Global<'a>>,
    exports: Vec<Export<'a>>,
    elements: Vec<Element<'a>>,
    datas: Vec<Data<'a>>,
    codes: Vec<FunctionBody<'a>>,
    customs: Vec<CustomSection<'a>>,
    // Handle: End, Unknown
}

impl<'a> Module<'a> {
    fn parse() -> Result<Self> {
        todo!()
    }

    pub fn from_binary(data: &[u8]) -> Result<Self> {
        todo!()
    }

    pub fn from_file(file: impl AsRef<Path>) -> Result<Self> {
        todo!()
    }
}
