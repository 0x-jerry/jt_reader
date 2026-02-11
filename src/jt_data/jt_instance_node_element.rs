use uuid::Uuid;

use crate::jt_data::{
    JtData, JtObjectTypeID, jt_base_node_data::JtBaseNodeData, jt_group_node_data::JtGroupNodeData,
};

#[derive(Debug, Default)]
pub struct JtInstanceNodeElement {
    pub base_node_data: JtBaseNodeData,
    pub version: i16,
    pub child_node_object_id: i32,
}

impl JtObjectTypeID for JtInstanceNodeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd102a,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtInstanceNodeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            base_node_data: JtBaseNodeData::read(reader)?,
            version: reader.read_i16()?,
            child_node_object_id: reader.read_i32()?,
        })
    }
}
