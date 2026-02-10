use anyhow::Result;
use uuid::Uuid;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug, Clone)]
pub struct JtTocEntry {
    pub segment_id: Uuid,
    pub offset: i32,
    pub length: i32,
    pub attributes: u32,
}

impl JtData for JtTocEntry {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let segment_id = reader.read_guid()?;
        let offset = reader.read_i32()?;
        let length = reader.read_i32()?;
        let attributes = reader.read_u32()?;
        Ok(Self {
            segment_id,
            offset,
            length,
            attributes,
        })
    }
}
