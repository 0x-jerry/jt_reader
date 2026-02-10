use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_point_quantizer_data::JtPointQuantizerData},
    jt_data_type::jt_vec::JtVecU32,
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtCompressedVertexCoordinateArray {
    pub unique_vertex_count: i32,
    pub number_components: u8,
    pub point_quantizer_data: JtPointQuantizerData,
    pub vertex_coord_exponents: Vec<JtVecU32>,
    pub vertex_coord_mantissae: Vec<JtVecU32>,
    pub vertex_coord_codes: Vec<JtVecU32>,
    pub vertex_coord_hash: i32,
}

impl JtData for JtCompressedVertexCoordinateArray {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();
        result.unique_vertex_count = reader.read_i32()?;
        result.number_components = reader.read_u8()?;
        result.point_quantizer_data = JtPointQuantizerData::read(reader)?;

        if result.point_quantizer_data.quant_bits() == 0 {
            for _ in 0..result.number_components {
                result.vertex_coord_exponents.push(JtVecU32::read(reader)?);
                result.vertex_coord_mantissae.push(JtVecU32::read(reader)?);
            }
        } else {
            for _ in 0..result.number_components {
                result.vertex_coord_codes.push(JtVecU32::read(reader)?);
            }
        }

        result.vertex_coord_hash = reader.read_i32()?;

        Ok(result)
    }
}
