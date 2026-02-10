use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_vertex_bindings::JtVertexBindings},
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtVertexShapeLODData {
    pub version: i16,
    pub vertex_bindings: JtVertexBindings,
}

impl JtData for JtVertexShapeLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let version = reader.read_i16()?;
        let vertex_bindings = JtVertexBindings::read(reader)?;

        Ok(Self {
            version,
            vertex_bindings,
        })
    }
}
