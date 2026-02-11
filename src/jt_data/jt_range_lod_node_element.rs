use uuid::Uuid;

use crate::{
    jt_data::{JtData, JtObjectTypeID, jt_lod_node_data::JtLODNodeData},
    jt_data_type::{jt_vec::JtVecF32, vec3::CoordF32},
};

#[derive(Debug, Default)]
pub struct JtRangeLODNodeElement {
    pub lod_node_data: JtLODNodeData,
    pub version: i16,
    pub range_limits: JtVecF32,
    pub center: CoordF32,
}

impl JtObjectTypeID for JtRangeLODNodeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd104c,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtRangeLODNodeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            lod_node_data: JtLODNodeData::read(reader)?,
            version: reader.read_i16()?,
            range_limits: reader.read_jt_vec_f32()?,
            center: reader.read_coordf32()?,
        })
    }
}
