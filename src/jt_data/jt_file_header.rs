use anyhow::{Result, anyhow};
use uuid::Uuid;

use crate::{
    jt_data::JtData,
    jt_reader::{ByteOrder, JtReader},
};

#[derive(Debug)]
pub struct JtFileHeader {
    pub version: String,
    pub byte_order: ByteOrder,
    pub reserved_field: i32,
    pub toc_offset: i32,
    pub lsg_segment_id: Uuid,
}

impl JtData for JtFileHeader {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let version = reader.read_string(80)?;
        let byte_order = reader.read_u8()?;
        let byte_order = match byte_order {
            0 => ByteOrder::LittleEndian,
            1 => ByteOrder::BigEndian,
            _ => return Err(anyhow!("Invalid byte order")),
        };

        let reserved_field = reader.read_i32()?;
        let toc_offset = reader.read_i32()?;
        let lsg_segment_id = reader.read_guid()?;

        Ok(Self {
            version,
            byte_order,
            reserved_field,
            toc_offset,
            lsg_segment_id,
        })
    }
}
