use crate::{
    jt_data::{
        JtData, jt_texture_quantizer_data::JtTextureQuantizerData,
        jt_uniform_quantizer_data::JtUniformQuantizerData,
    },
    jt_data_type::jt_vec::JtVecU32,
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtCompressedVertexTextureCoordinateArray {
    pub texture_coordinate_count: i32,
    pub number_of_components: u8,
    pub quantization_bits: u8,

    pub vertex_texture_coord_exponents: Vec<JtVecU32>,
    pub vertex_texture_coord_mantissae: Vec<JtVecU32>,

    pub texture_quantizer_data: Vec<JtUniformQuantizerData>,
    pub texture_coord_codes: Vec<JtVecU32>,

    pub vertex_texture_coord_hash: u32,
}

impl JtData for JtCompressedVertexTextureCoordinateArray {
    fn read(reader: &mut JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();

        result.texture_coordinate_count = reader.read_i32()?;
        result.number_of_components = reader.read_u8()?;
        result.quantization_bits = reader.read_u8()?;

        if result.quantization_bits == 0 {
            for _ in 0..result.number_of_components {
                result
                    .vertex_texture_coord_exponents
                    .push(JtVecU32::read(reader)?);
                result
                    .vertex_texture_coord_mantissae
                    .push(JtVecU32::read(reader)?);
            }
        } else {
            result.texture_quantizer_data =
                JtTextureQuantizerData::read(reader, result.number_of_components as u32)?.data;
            for _ in 0..result.number_of_components {
                result.texture_coord_codes.push(JtVecU32::read(reader)?);
            }
        }

        result.vertex_texture_coord_hash = reader.read_u32()?;

        Ok(result)
    }
}
