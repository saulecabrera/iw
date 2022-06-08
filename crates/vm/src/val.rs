use crate::addressable::Addr;

use anyhow::bail;
use wasmparser::Type;

#[derive(PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    RefType(RefType),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    Ref(RefValue),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RefValue {
    FuncRef(Addr),
    // TODO: Fill in once imports are supported
    ExternRef,
    Null(RefType),
}

impl RefValue {
    pub fn ty(&self) -> RefType {
        match self {
            RefValue::FuncRef(_) => RefType::FuncRef,
            RefValue::ExternRef => RefType::ExternRef,
            RefValue::Null(t) => *t,
        }
    }

    pub fn is_func_ref(&self) -> bool {
        self.ty() == RefType::FuncRef
    }
}

impl Value {
    pub fn ty(&self) -> ValueType {
        match &*self {
            Value::I32(_) => ValueType::I32,
            Value::I64(_) => ValueType::I64,
            Value::F32(_) => ValueType::F32,
            Value::F64(_) => ValueType::F64,
            Value::Ref(v) => match v {
                RefValue::FuncRef(_) => ValueType::RefType(RefType::FuncRef),
                RefValue::ExternRef => ValueType::RefType(RefType::ExternRef),
                RefValue::Null(t) => ValueType::RefType(t.clone()),
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

impl TryFrom<Type> for RefType {
    type Error = anyhow::Error;

    fn try_from(ty: Type) -> anyhow::Result<RefType> {
        match ty {
            Type::ExternRef => Ok(RefType::ExternRef),
            Type::FuncRef => Ok(RefType::FuncRef),
            _type => bail!("expected FuncRef or ExternRef, got: {:?}", _type),
        }
    }
}
