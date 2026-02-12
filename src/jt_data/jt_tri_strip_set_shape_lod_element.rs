use anyhow::bail;
use uuid::Uuid;

use crate::{
    jt_data::{
        JtData, JtObjectTypeID, jt_base_shape_lod_data::JtBaseShapeLODData,
        jt_vertex_shape_lod_data::JtVertexShapeLODData,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtTriStripSetShapeLODElement {
    // Vertex Shape LOD Data
    pub vertex_shape_lod_data: JtVertexShapeLODData,
    pub version: i16,
}

impl JtObjectTypeID for JtTriStripSetShapeLODElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd10ab,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtTriStripSetShapeLODElement {
    fn read(reader: &mut JtReader) -> anyhow::Result<Self> {
        let vertex_shape_lod_data = JtVertexShapeLODData::read(reader)?;
        let version = reader.read_i16()?;

        if version != 1 {
            bail!("Invalid JtTriStripSetShapeLODElement version")
        }

        Ok(Self {
            vertex_shape_lod_data,
            version,
        })
    }
}
