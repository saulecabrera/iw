use anyhow::{bail, Result};
use wasmparser::*;

type Index = u32;

struct CustomSection<'a> {
    name: &'a str,
    data_offset: usize,
    data: &'a [u8],
    range: Range,
}

pub struct Module<'a> {
    // NB
    // The custom name section can be used to obtain
    // function names. `NameSectionReader` can be used
    // to retrieve function names and `ExportSectionReader`
    // can be used to retrieve export names.
    // Ref: https://github.com/bytecodealliance/wasm-tools/issues/104
    //
    start_fn_idx: Option<Index>,

    types: Vec<FuncType>,
    imports: Vec<Import<'a>>,
    functions: Vec<Index>,
    tables: Vec<TableType>,
    memories: Vec<MemoryType>,
    pub globals: Vec<Global<'a>>,
    exports: Vec<Export<'a>>,
    elements: Vec<Element<'a>>,
    datas: Vec<Data<'a>>,
    codes: Vec<FunctionBody<'a>>,
    customs: Vec<CustomSection<'a>>,
    // TODO
    // - What is the purpose of the UnknownSection? Should we error if we encounter it?
}

impl<'a> Default for Module<'a> {
    fn default() -> Self {
        Self {
            start_fn_idx: None,
            types: Vec::new(),
            imports: Vec::new(),
            functions: Vec::new(),
            tables: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),
            exports: Vec::new(),
            elements: Vec::new(),
            datas: Vec::new(),
            codes: Vec::new(),
            customs: Vec::new(),
        }
    }
}

impl<'a> Module<'a> {
    fn parse(data: &'a [u8]) -> Result<Self> {
        let parser = Parser::new(0);
        let mut parsed = parser.parse_all(data);
        let acc = Module::default();

        parsed.try_fold(acc, Self::map_payload)
    }

    pub fn from_binary(data: &'a [u8]) -> Result<Self> {
        Self::parse(data)
    }

    pub fn func_types(&self) -> Vec<FuncType> {
        self.types.clone()
    }

    fn map_payload(
        mut module: Module<'a>,
        payload: Result<Payload<'a>, BinaryReaderError>,
    ) -> Result<Module<'a>> {
        match payload? {
            Payload::TypeSection(reader) => {
                let count = reader.get_count() as usize;
                let func_types = Self::parse_type_section(reader)?;
                module.types.reserve_exact(count);
                module.types = func_types;
            }

            Payload::ImportSection(reader) => {
                let count = reader.get_count() as usize;
                let imports = Self::parse_import_section(reader)?;
                module.imports.reserve_exact(count);
                module.imports = imports;
            }

            Payload::AliasSection(_) => bail!("Alias section not supported"),
            Payload::InstanceSection(_) => bail!("Instance section not supported"),
            Payload::ModuleSectionStart { .. } => bail!("Nested modules are not supported"),
            Payload::ModuleSectionEntry { .. } => bail!("Nested modules are not supported"),

            Payload::FunctionSection(reader) => {
                let count = reader.get_count() as usize;
                let functions = Self::parse_function_section(reader)?;
                module.functions.reserve_exact(count);
                module.functions = functions;
            }

            Payload::TableSection(reader) => {
                let count = reader.get_count() as usize;
                let tables = Self::parse_table_section(reader)?;
                module.tables.reserve_exact(count);
                module.tables = tables;
            }

            Payload::MemorySection(reader) => {
                let count = reader.get_count() as usize;
                let memories = Self::parse_memory_section(reader)?;
                module.memories.reserve_exact(count);
                module.memories = memories;
            }

            Payload::TagSection(_) => bail!("Tag section not supported"),

            Payload::GlobalSection(reader) => {
                let count = reader.get_count() as usize;
                let globals = Self::parse_global_section(reader)?;
                module.globals.reserve_exact(count);
                module.globals = globals;
            }

            Payload::ExportSection(reader) => {
                let count = reader.get_count() as usize;
                let exports = Self::parse_export_section(reader)?;
                module.exports.reserve_exact(count);
                module.exports = exports;
            }

            Payload::StartSection { func, .. } => {
                let start_fn_idx = func as Index;
                module.start_fn_idx = Some(start_fn_idx);
            }

            Payload::ElementSection(reader) => {
                let count = reader.get_count() as usize;
                let elements = Self::parse_element_section(reader)?;
                module.elements.reserve_exact(count);
                module.elements = elements;
            }

            Payload::DataSection(reader) => {
                let count = reader.get_count() as usize;
                let datas = Self::parse_data_section(reader)?;
                module.datas.reserve_exact(count);
                module.datas = datas;
            }

            Payload::CodeSectionStart { count, .. } => {
                module.codes.reserve_exact(count as usize);
            }

            Payload::CodeSectionEntry(func) => {
                module.codes.push(func);
            }

            Payload::CustomSection {
                name,
                data,
                data_offset,
                range,
            } => {
                let custom_section = CustomSection {
                    name,
                    data_offset,
                    data,
                    range,
                };

                module.customs.push(custom_section);
            }

            // TODO
            // - Potentially record function names from the name custom section
            // - Track the starting point of the code section
            _ => (),
        }

        Ok(module)
    }

    fn parse_type_section(reader: TypeSectionReader<'a>) -> Result<Vec<FuncType>> {
        reader.into_iter().try_fold(Vec::new(), |mut types, def| {
            match def? {
                TypeDef::Func(ty) => {
                    types.push(ty);
                }
                TypeDef::Module(_) => bail!("Module type not supported"),
                TypeDef::Instance(_) => bail!("Instance type not supported"),
            }

            Ok(types)
        })
    }

    fn parse_import_section(reader: ImportSectionReader<'a>) -> Result<Vec<Import<'a>>> {
        reader.into_iter().try_fold(Vec::new(), |mut imports, def| {
            let import = def?;
            match import.ty {
                ImportSectionEntryType::Tag(_) => bail!("Tag import not supported"),
                ImportSectionEntryType::Instance(_) => bail!("Instance import not supported"),
                ImportSectionEntryType::Module(_) => bail!("Module import not supported"),
                _ => imports.push(import),
            }

            Ok(imports)
        })
    }

    fn parse_function_section(reader: FunctionSectionReader<'a>) -> Result<Vec<Index>> {
        reader
            .into_iter()
            .try_fold(Vec::new(), |mut functions, index| {
                functions.push(index?);
                Ok(functions)
            })
    }

    fn parse_table_section(reader: TableSectionReader<'a>) -> Result<Vec<TableType>> {
        reader.into_iter().try_fold(Vec::new(), |mut tables, ty| {
            tables.push(ty?);
            Ok(tables)
        })
    }

    fn parse_memory_section(reader: MemorySectionReader<'a>) -> Result<Vec<MemoryType>> {
        reader.into_iter().try_fold(Vec::new(), |mut memories, ty| {
            memories.push(ty?);
            Ok(memories)
        })
    }

    fn parse_global_section(reader: GlobalSectionReader<'a>) -> Result<Vec<Global<'a>>> {
        reader
            .into_iter()
            .try_fold(Vec::new(), |mut globals, global| {
                globals.push(global?);
                Ok(globals)
            })
    }

    fn parse_export_section(reader: ExportSectionReader<'a>) -> Result<Vec<Export<'a>>> {
        reader.into_iter().try_fold(Vec::new(), |mut exports, def| {
            exports.push(def?);
            Ok(exports)
        })
    }

    fn parse_element_section(reader: ElementSectionReader<'a>) -> Result<Vec<Element<'a>>> {
        reader
            .into_iter()
            .try_fold(Vec::new(), |mut elements, element| {
                elements.push(element?);
                Ok(elements)
            })
    }

    fn parse_data_section(reader: DataSectionReader<'a>) -> Result<Vec<Data<'a>>> {
        reader.into_iter().try_fold(Vec::new(), |mut datas, data| {
            datas.push(data?);
            Ok(datas)
        })
    }
}
