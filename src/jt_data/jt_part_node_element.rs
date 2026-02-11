use uuid::Uuid;

use crate::jt_data::{
    JtData, JtObjectTypeID, jt_meta_data_node_element::JtMetaDataNodeData,
};

#[derive(Debug, Default)]
pub struct JtPartNodeElement {
    pub meta_data_node_data: JtMetaDataNodeData,
    pub version: i16,
    pub reserved_field: i32,
}

impl JtObjectTypeID for JtPartNodeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0xce357244,
        0x38fb,
        0x11d1,
        &[0xa5, 0x6, 0x0, 0x60, 0x97, 0xbd, 0xc6, 0xe1],
    );
}

impl JtData for JtPartNodeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            meta_data_node_data: JtMetaDataNodeData::read(reader)?,
            version: reader.read_i16()?,
            reserved_field: reader.read_i32()?,
        })
    }
}
