use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_uniform_quantizer_data::JtUniformQuantizerData},
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtPointQuantizerData {
    pub x_uniform_quantizer_data: JtUniformQuantizerData,
    pub y_uniform_quantizer_data: JtUniformQuantizerData,
    pub z_uniform_quantizer_data: JtUniformQuantizerData,
}

impl JtPointQuantizerData {
    pub fn quant_bits(&self) -> u8 {
        // all components should have the same number of bits
        self.x_uniform_quantizer_data.number_of_bits
    }
}

impl JtData for JtPointQuantizerData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        Ok(Self {
            x_uniform_quantizer_data: JtUniformQuantizerData::read(reader)?,
            y_uniform_quantizer_data: JtUniformQuantizerData::read(reader)?,
            z_uniform_quantizer_data: JtUniformQuantizerData::read(reader)?,
        })
    }
}
