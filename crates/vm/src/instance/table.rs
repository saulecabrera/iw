use crate::{
    addressable::{Slot, Slottable},
    val::{RefType, RefValue, Value},
};
use anyhow::{bail, Result};

pub struct Table {
    buffer: Vec<RefValue>,
    ty: RefType,
    initial: u32,
    maximum: Option<u32>,
}

impl Table {
    pub fn new(ty: RefType, initial: u32, maximum: Option<u32>) -> Result<Self> {
        Ok(Self {
            buffer: std::iter::repeat(RefValue::Null(ty))
                .take(initial as usize)
                .collect(),
            ty,
            initial,
            maximum,
        })
    }

    fn validate_offset(&self, offset: usize) -> Result<()> {
        let size = self.buffer.len();
        let out_of_bounds = || {
            bail!(
                "Table out of bounds, tried to access {}, but table has length {}",
                offset,
                size
            )
        };
        match offset.checked_add(size) {
            Some(addr) => {
                if addr <= size {
                    return Ok(());
                }
                out_of_bounds()
            }
            None => out_of_bounds(),
        }
    }

    pub fn init(&mut self, val: &Value, data: Vec<RefValue>) -> Result<()> {
        let offset = match val {
            Value::I32(v) => usize::try_from(*v),
            Value::I64(v) => usize::try_from(*v),
            v => bail!("Unexpected value in table initialization: {:?}, expected ValueType::I32 or ValueType::I64", v),
        }?;

        self.validate_offset(offset)?;

        for (index, ref_val) in data.iter().enumerate() {
            self.buffer[offset + index] = *ref_val;
        }

        Ok(())
    }
}

impl Slottable for Table {
    fn slot() -> Slot {
        Slot::Table
    }
}
