use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug)]
pub struct JtBaseShapeLODData {
    pub version: i16,
}

impl JtData for JtBaseShapeLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let version = reader.read_i16()?;

        Ok(Self { version })
    }
}
