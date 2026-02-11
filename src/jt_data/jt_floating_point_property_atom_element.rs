use uuid::Uuid;

use crate::jt_data::{JtData, JtObjectTypeID, jt_base_property_atom_data::JtBasePropertyAtomData};

#[derive(Debug, Default)]
pub struct JtFloatingPointPropertyAtomElement {
    pub base_property_data: JtBasePropertyAtomData,
    pub version: i16,
    pub value: f32,
}

impl JtObjectTypeID for JtFloatingPointPropertyAtomElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd1019,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtFloatingPointPropertyAtomElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            base_property_data: JtBasePropertyAtomData::read(reader)?,
            version: reader.read_i16()?,
            value: reader.read_f32()?,
        })
    }
}
