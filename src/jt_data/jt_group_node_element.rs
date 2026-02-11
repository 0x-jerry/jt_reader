use uuid::Uuid;

use crate::jt_data::{JtData, JtObjectTypeID, jt_group_node_data::JtGroupNodeData};

#[derive(Debug, Default)]
pub struct JtGroupNodeElement {
    pub group_node_data: JtGroupNodeData,
}

impl JtObjectTypeID for JtGroupNodeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd101b,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtGroupNodeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            group_node_data: JtGroupNodeData::read(reader)?,
        })
    }
}
