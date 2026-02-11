use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_base_node_data::JtBaseNodeData, jt_count_range::JtCountRange},
    jt_data_type::bboxf32::BBoxF32,
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtBaseShapeData {
    pub base_node_data: JtBaseNodeData,
    pub version: i16,
    pub reserved_field: BBoxF32,
    pub untransformed_bbox: BBoxF32,
    pub area: f32,
    pub vertex_count_range: JtCountRange,
    pub node_count_range: JtCountRange,
    pub polygon_count_range: JtCountRange,
    pub size: i32,
    pub compression_level: f32,
}

impl JtData for JtBaseShapeData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        Ok(Self {
            base_node_data: JtBaseNodeData::read(reader)?,
            version: reader.read_i16()?,
            reserved_field: BBoxF32::read(reader)?,
            untransformed_bbox: BBoxF32::read(reader)?,
            area: reader.read_f32()?,
            vertex_count_range: JtCountRange::read(reader)?,
            node_count_range: JtCountRange::read(reader)?,
            polygon_count_range: JtCountRange::read(reader)?,
            size: reader.read_i32()?,
            compression_level: reader.read_f32()?,
        })
    }
}
