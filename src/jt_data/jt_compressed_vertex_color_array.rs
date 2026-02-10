use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_color_quantizer_data::JtColorQuantizerData},
    jt_data_type::jt_vec::JtVecU32,
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtCompressedVertexColorArray {
    pub color_count: i32,
    pub number_components: u8,
    pub quantization_bits: u8,
    pub vertex_color_exponents: Vec<JtVecU32>,
    pub vertex_color_mantissae: Vec<JtVecU32>,
    pub color_quantizer_data: JtColorQuantizerData,
    pub hue_or_red_codes: JtVecU32,
    pub sat_or_green_codes: JtVecU32,
    pub value_or_blue_codes: JtVecU32,
    pub alpha_codes: JtVecU32,
    pub vertex_color_hash: i32,
}

impl JtData for JtCompressedVertexColorArray {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        result.color_count = reader.read_i32()?;
        result.number_components = reader.read_u8()?;
        result.quantization_bits = reader.read_u8()?;

        if result.quantization_bits == 0 {
            for _ in 0..result.number_components {
                result.vertex_color_exponents.push(JtVecU32::read(reader)?);
                result.vertex_color_mantissae.push(JtVecU32::read(reader)?);
            }
        } else {
            result.color_quantizer_data = JtColorQuantizerData::read(reader)?;
            result.hue_or_red_codes = JtVecU32::read(reader)?;
            result.sat_or_green_codes = JtVecU32::read(reader)?;
            result.value_or_blue_codes = JtVecU32::read(reader)?;
            result.alpha_codes = JtVecU32::read(reader)?;
        }

        result.vertex_color_hash = reader.read_i32()?;
        Ok(result)
    }
}
