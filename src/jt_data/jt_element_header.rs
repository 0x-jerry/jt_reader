use anyhow::Result;
use uuid::Uuid;

use crate::{
    jt_data::{JtData, jt_base_type::JtBaseType},
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtElementHeader {
    pub object_type_id: Uuid,
    pub object_base_type: JtBaseType,
    pub object_id: i32,
}

impl JtData for JtElementHeader {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let type_id = reader.read_guid()?;
        let base_type = JtBaseType::new(reader.read_u8()?);
        let id = reader.read_i32()?;

        Ok(Self {
            object_type_id: type_id,
            object_base_type: base_type,
            object_id: id,
        })
    }
}
