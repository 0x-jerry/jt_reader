use anyhow::Result;

use crate::{
    jt_data::{
        JtData, jt_topologically_compressed_vertex_records::JtTopologicallyCompressedVertexRecords,
    },
    jt_data_type::jt_vec::{JtVecI32, JtVecU32},
    jt_reader::JtReader,
};

#[derive(Debug)]
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
        let mut face_degrees: [JtVecI32; 8] = Default::default();

        // todo: those values are compressed value, need to decompress

        for i in 0..8 {
            face_degrees[i] = JtVecI32::read_by_cdp(reader)?;
        }

        let vertex_valences = reader.read_jt_vec_i32()?;

        let vertex_groups = reader.read_jt_vec_i32()?;
        let vertex_flags = reader.read_jt_vec_i32()?;

        let mut face_attribute_masks: [JtVecI32; 8] = Default::default();

        for i in 0..8 {
            face_attribute_masks[i] = reader.read_jt_vec_i32()?;
        }

        let face_attribute_mask_8_30_next_msbs = reader.read_jt_vec_i32()?;
        let face_attribute_mask_8_4_msbs = reader.read_jt_vec_i32()?;

        let high_degree_face_attribute_masks = reader.read_jt_vec_u32()?;

        let split_face_syms = reader.read_jt_vec_i32()?;
        let split_face_positions = reader.read_jt_vec_i32()?;

        let composite_hash = reader.read_u32()?;

        let topologically_compressed_vertex_records =
            JtTopologicallyCompressedVertexRecords::read(reader)?;

        return Ok(Self {
            face_degrees,
            vertex_valences,
            vertex_groups,
            vertex_flags,
            face_attribute_masks,
            face_attribute_mask_8_30_next_msbs,
            face_attribute_mask_8_4_msbs,
            high_degree_face_attribute_masks,
            split_face_syms,
            split_face_positions,
            composite_hash,
            topologically_compressed_vertex_records,
        });
    }
}
