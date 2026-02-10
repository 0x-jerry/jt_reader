use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug, Default)]
pub struct JtUniformQuantizerData {
    pub min: f32,
    pub max: f32,
    pub number_of_bits: u8,
}

impl JtData for JtUniformQuantizerData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let min = reader.read_f32()?;
        let max = reader.read_f32()?;
        let number_of_bits = reader.read_u8()?;

        Ok(Self {
            min,
            max,
            number_of_bits,
        })
    }
}
