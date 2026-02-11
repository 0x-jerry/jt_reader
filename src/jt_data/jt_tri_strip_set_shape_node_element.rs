use uuid::Uuid;

use crate::jt_data::{JtData, JtObjectTypeID, jt_vertex_shape_data::JtVertexShapeData};

#[derive(Debug, Default)]
pub struct JtTriStripSetShapeNodeElement {
    pub vertex_shape_data: JtVertexShapeData,
}

impl JtObjectTypeID for JtTriStripSetShapeNodeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd1077,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtTriStripSetShapeNodeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            vertex_shape_data: JtVertexShapeData::read(reader)?,
        })
    }
}
