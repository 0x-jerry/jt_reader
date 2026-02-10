use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_uniform_quantizer_data::JtUniformQuantizerData},
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtTextureQuantizerData {
    pub data: Vec<JtUniformQuantizerData>,
}

impl JtTextureQuantizerData {
    pub fn read(reader: &mut JtReader, count: u32) -> Result<Self> {
        let mut result: Self = Default::default();

        for _ in 0..count {
            result.data.push(JtUniformQuantizerData::read(reader)?);
        }

        Ok(result)
    }
}
