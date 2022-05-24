use crate::addressable::Addressable;
use crate::instance::Func;

use anyhow::bail;
use wasmparser::Type;

pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    RefType(RefType),
    NullRef(RefType),
}

pub enum RefType {
    FuncRef,
    ExternRef,
}

pub enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    NullRef(RefValue),
    Ref(RefValue),
}

pub enum RefValue {
    FuncRef(Addressable<Func>),
    // TODO: Fill in once imports are supported
    ExternRef,
}

impl Value {
    pub fn ty(&self) -> ValueType {
        match *self {
            Value::I32(_) => ValueType::I32,
            Value::I64(_) => ValueType::I64,
            Value::F32(_) => ValueType::F32,
            Value::F64(_) => ValueType::F64,
            Value::NullRef(ref r) => match r {
                RefValue::ExternRef => ValueType::NullRef(RefType::ExternRef),
                RefValue::FuncRef(_) => ValueType::NullRef(RefType::FuncRef),
            },
            Value::Ref(ref v) => match v {
                RefValue::FuncRef(_) => ValueType::RefType(RefType::FuncRef),
                RefValue::ExternRef => ValueType::RefType(RefType::ExternRef),
            },
        }
    }
}

impl TryFrom<Type> for ValueType {
    type Error = anyhow::Error;

    fn try_from(ty: Type) -> anyhow::Result<ValueType> {
        let t = match ty {
            Type::I32 => ValueType::I32,
            Type::I64 => ValueType::I64,
            Type::F32 => ValueType::F32,
            Type::F64 => ValueType::F64,
            Type::ExternRef => ValueType::RefType(RefType::ExternRef),
            Type::FuncRef => ValueType::RefType(RefType::FuncRef),
            type_ => bail!("{:?} type not supported", type_),
        };

        Ok(t)
    }
}
