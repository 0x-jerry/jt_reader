use uuid::Uuid;

use crate::jt_data::{JtData, JtObjectTypeID, jt_group_node_data::JtGroupNodeData};

#[derive(Debug, Default)]
pub struct JtMetaDataNodeElement {
    pub group_node_data: JtGroupNodeData,
    pub version: i16,
}

pub type JtMetaDataNodeData = JtMetaDataNodeElement;

impl JtObjectTypeID for JtMetaDataNodeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0xce357245,
        0x38fb,
        0x11d1,
        &[0xa5, 0x6, 0x0, 0x60, 0x97, 0xbd, 0xc6, 0xe1],
    );
}

impl JtData for JtMetaDataNodeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            group_node_data: JtGroupNodeData::read(reader)?,
            version: reader.read_i16()?,
        })
    }
}
