use anyhow::Result;
use uuid::Uuid;

use crate::{
    jt_data::{JtData, jt_base_type::JtBaseType, jt_common_marker::END_OF_ELEMENTS_MARKER},
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtElementHeader {
    pub object_type_id: Uuid,
    pub object_base_type: JtBaseType,
    pub object_id: i32,
}

impl JtElementHeader {
    pub fn is_end_marker_object_type(&self) -> bool {
        self.object_type_id == END_OF_ELEMENTS_MARKER
    }
}

impl JtData for JtElementHeader {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        result.object_type_id = reader.read_guid()?;

        if result.object_type_id == END_OF_ELEMENTS_MARKER {
            return Ok(result);
        }

        result.object_base_type = JtBaseType::new(reader.read_u8()?);
        result.object_id = reader.read_i32()?;

        Ok(result)
    }
}
