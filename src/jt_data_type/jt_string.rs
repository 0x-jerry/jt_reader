use anyhow::{Ok, Result};

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug)]
pub struct JtString {
    pub count: i32,
    pub string: String,
}

impl JtData for JtString {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let count = reader.read_i32()?;
        let str = reader.read_string(count as usize)?;

        Ok(Self { count, string: str })
    }
}
