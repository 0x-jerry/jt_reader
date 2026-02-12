use anyhow::Result;

use crate::{
    jt_data::JtData, jt_data_type::jt_vec::JtVecU32, jt_decode::jt_cdp2::JtCDP2Data,
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtCompressedVertexNormalArray {
    pub normal_count: i32,
    pub number_components: u8,
    pub quantization_bits: u8,
    pub vertex_normal_exponents: Vec<JtVecU32>,
    pub vertex_normal_mantissae: Vec<JtVecU32>,
    pub sextant_codes: JtVecU32,
    pub octant_codes: JtVecU32,
    pub theta_codes: JtVecU32,
    pub psi_codes: JtVecU32,
    pub vertex_normal_hash: i32,
}

impl JtData for JtCompressedVertexNormalArray {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        result.normal_count = reader.read_i32()?;
        result.number_components = reader.read_u8()?;
        result.quantization_bits = reader.read_u8()?;

        if result.quantization_bits == 0 {
            for _ in 0..result.number_components {
                // todo, fix type
                // result.vertex_normal_exponents.push(JtVecU32::read(reader)?);
                // result.vertex_normal_mantissae.push(JtVecU32::read(reader)?);

                JtCDP2Data::read(reader)?;
                JtCDP2Data::read(reader)?;
            }
        } else {
            // todo, fix type
            // result.sextant_codes = JtVecU32::read(reader)?;
            // result.octant_codes = JtVecU32::read(reader)?;
            // result.theta_codes = JtVecU32::read(reader)?;
            // result.psi_codes = JtVecU32::read(reader)?;

            JtCDP2Data::read(reader)?;
            JtCDP2Data::read(reader)?;
            JtCDP2Data::read(reader)?;
            JtCDP2Data::read(reader)?;
        }

        result.vertex_normal_hash = reader.read_i32()?;

        Ok(result)
    }
}
