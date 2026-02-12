use crate::{
    jt_data::JtData, jt_data_type::jt_vec::JtVecU32, jt_decode::jt_cdp2::JtCDP2Data,
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtCompressedVertexFlagArray {
    pub vetex_flag_count: u32,
    pub vertex_flags: JtVecU32,
}

impl JtData for JtCompressedVertexFlagArray {
    fn read(reader: &mut JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();

        result.vetex_flag_count = reader.read_u32()?;
        // todo, fix type
        // result.vertex_flags = JtVecU32::read(reader)?;

        JtCDP2Data::read(reader)?;

        Ok(result)
    }
}
