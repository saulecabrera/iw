pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

pub enum Value {
    // -- Numeric
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),

    // TODO
    // -- Reference
    // NullRef,
    // FuncRef,
    // ExternRef,
}

impl Value {
    pub fn ty(&self) -> ValueType 
    {
        match *self {
            Value::I32(_) => ValueType::I32,
            Value::I64(_) => ValueType::I64,
            Value::F32(_) => ValueType::F32,
            Value::F64(_) => ValueType::F64,
        }
    }
}

