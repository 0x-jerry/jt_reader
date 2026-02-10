use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug)]
pub struct JtTopoMeshLODData {
    pub version: i16,
    pub vertex_records_object_id: i32,
}

impl JtData for JtTopoMeshLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        Ok(Self {
            version: reader.read_i16()?,
            vertex_records_object_id: reader.read_i32()?,
        })
    }
}
