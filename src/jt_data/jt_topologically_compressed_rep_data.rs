use anyhow::{Result, bail};

use crate::{
    jt_data::{
        JtData, jt_topologically_compressed_vertex_records::JtTopologicallyCompressedVertexRecords,
    },
    jt_data_type::jt_vec::{JtVecI32, JtVecU32},
    jt_decode::jt_cdp2::JtCDP2Data,
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtTopologicallyCompressedRepData {
    face_degrees: [JtVecI32; 8],
    vertex_valences: JtVecI32,
    vertex_groups: JtVecI32,
    vertex_flags: JtVecI32,
    face_attribute_masks: [JtVecI32; 8],
    face_attribute_mask_8_30_next_msbs: JtVecI32,
    face_attribute_mask_8_4_msbs: JtVecI32,
    high_degree_face_attribute_masks: JtVecU32,
    split_face_syms: JtVecI32,
    split_face_positions: JtVecI32,
    composite_hash: u32,
    topologically_compressed_vertex_records: JtTopologicallyCompressedVertexRecords,
}

impl JtData for JtTopologicallyCompressedRepData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let result: Self = Default::default();
        let mut face_degrees: [JtVecI32; 8] = Default::default();

        for i in 0..8 {
            // face_degrees[i] = JtVecI32::read_by_cdp_v2(reader)?;
            JtCDP2Data::read(reader)?;
        }

        // let vertex_valences = JtVecI32::read_by_cdp_v2(reader)?;
        JtCDP2Data::read(reader)?;

        // let vertex_groups = JtVecI32::read_by_cdp_v2(reader)?;
        JtCDP2Data::read(reader)?;

        // let vertex_flags = JtVecI32::read_by_cdp_v2(reader)?;
        JtCDP2Data::read(reader)?;

        let mut face_attribute_masks: [JtVecI32; 8] = Default::default();

        for i in 0..8 {
            // face_attribute_masks[i] = JtVecI32::read_by_cdp_v2(reader)?;
            JtCDP2Data::read(reader)?;
        }

        // let face_attribute_mask_8_30_next_msbs = JtVecI32::read_by_cdp_v2(reader)?;
        // let face_attribute_mask_8_4_msbs = JtVecI32::read_by_cdp_v2(reader)?;
        JtCDP2Data::read(reader)?;
        JtCDP2Data::read(reader)?;

        let high_degree_face_attribute_masks = reader.read_jt_vec_u32()?;

        // let split_face_syms = JtVecI32::read_by_cdp_v2(reader)?;
        // let split_face_positions = JtVecI32::read_by_cdp_v2(reader)?;
        JtCDP2Data::read(reader)?;
        JtCDP2Data::read(reader)?;

        let composite_hash = reader.read_u32()?;

        let topologically_compressed_vertex_records =
            JtTopologicallyCompressedVertexRecords::read(reader)?;

        return Ok(result);
    }
}
