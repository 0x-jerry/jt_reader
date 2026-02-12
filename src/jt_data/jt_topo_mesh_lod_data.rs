use anyhow::{Result, bail};

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug)]
pub struct JtTopoMeshLODData {
    pub version: i16,
    pub vertex_records_object_id: i32,
}

impl JtData for JtTopoMeshLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let version = reader.read_i16()?;

        if version != 1 && version != 2 {
            bail!("Invalid JtTopoMeshLODData version {}", version)
        }

        Ok(Self {
            version,
            vertex_records_object_id: reader.read_i32()?,
        })
    }
}
