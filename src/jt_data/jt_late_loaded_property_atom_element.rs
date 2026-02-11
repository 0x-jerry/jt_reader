use uuid::Uuid;

use crate::jt_data::{JtData, JtObjectTypeID, jt_base_property_atom_data::JtBasePropertyAtomData};

#[derive(Debug, Default)]
pub struct JtLateLoadedPropertyAtomElement {
    pub base_property_data: JtBasePropertyAtomData,
    pub version: i16,
    pub segment_id: Uuid,
    pub segment_type: i32,
    pub payload_object_id: i32,
    pub rserved: i32,
}

impl JtObjectTypeID for JtLateLoadedPropertyAtomElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0xe0b05be5,
        0xfbbd,
        0x11d1,
        &[0xa3, 0xa7, 0x00, 0xaa, 0x00, 0xd1, 0x09, 0x54],
    );
}

impl JtData for JtLateLoadedPropertyAtomElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            base_property_data: JtBasePropertyAtomData::read(reader)?,
            version: reader.read_i16()?,
            segment_id: reader.read_guid()?,
            segment_type: reader.read_i32()?,
            payload_object_id: reader.read_i32()?,
            rserved: reader.read_i32()?,
        })
    }
}
