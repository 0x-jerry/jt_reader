use anyhow::{Ok, Result};

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug, Default)]
pub struct MbString {
    pub count: i32,
    pub string: String,
}

impl JtData for MbString {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let count = reader.read_i32()?;
        let mut buf = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let c = reader.read_u16()?;
            buf.push(c);
        }

        let str = String::from_utf16_lossy(&buf);

        Ok(Self { count, string: str })
    }
}
