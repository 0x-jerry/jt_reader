use anyhow::Result;
use uuid::Uuid;

use crate::{jt_data::{JtData, jt_segment_type::SegmentType}, jt_reader::JtReader};

#[derive(Debug)]
pub struct JtSegmentHeader {
    pub id: Uuid,
    pub s_type: SegmentType,
    pub length: i32,
}

impl JtData for JtSegmentHeader {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let segment_id = reader.read_guid()?;
        let segment_type = reader.read_i32()?;
        let segment_type = SegmentType::new(segment_type);
        let segment_length = reader.read_i32()?;

        Ok(Self {
            id: segment_id,
            s_type: segment_type,
            length: segment_length,
        })
    }
}
