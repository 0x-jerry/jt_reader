use anyhow::{Result, anyhow};
use uuid::Uuid;

use crate::{
    jt_data::JtData,
    jt_reader::{ByteOrder, JtReader},
};

#[derive(Debug)]
pub struct JtFileHeader {
    pub version: String,
    pub major_version: i32,
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

        let parsed_version = parse_version_str(&version)?;

        Ok(Self {
            version,
            major_version: parsed_version.0,
            byte_order,
            reserved_field,
            toc_offset,
            lsg_segment_id,
        })
    }
}

fn parse_version_str(version: &String) -> Result<(i32, i32)> {
    let re = regex::Regex::new(r"^\s*Version\s+(\d+)\.(\d+)\s")?;
    let caps = re.captures(version).expect("Parse version failed!");

    let major = caps[1].parse::<i32>()?;
    let minor = caps[2].parse::<i32>()?;

    Ok((major, minor))
}
