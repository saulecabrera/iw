pub enum Value {
    // -- Numeric
    I32(i32),
    I64(i64),
    F32(F32),
    F64(F64),

    // -- Reference
    NullRef,
    FuncRef,
    ExternRef,
}

pub struct F32(u32);
pub struct F64(u64);
