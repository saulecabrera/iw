use wasmparser::Validator;

pub struct ModuleEnv<'a> {
    validator: &'a mut Validator,
}
