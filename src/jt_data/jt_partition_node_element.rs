use anyhow::Result;
use uuid::Uuid;

use crate::{
    jt_data::{JtData, JtObjectTypeID, jt_group_node_data::JtGroupNodeData},
    jt_data_type::{bboxf32::BBoxF32, mbstring::MbString},
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtCountRange {
    pub min: i32,
    pub max: i32,
}

impl JtData for JtCountRange {
    fn read(reader: &mut JtReader) -> Result<Self> {
        Ok(Self {
            min: reader.read_i32()?,
            max: reader.read_i32()?,
        })
    }
}

#[derive(Debug, Default)]
pub struct JtPartitionNodeElement {
    pub group_node_data: JtGroupNodeData,
    pub partition_flags: i32,
    pub file_name: MbString,
    pub transformed_bbox: BBoxF32,
    pub area: f32,
    pub vertex_count_range: JtCountRange,
    pub node_count_range: JtCountRange,
    pub polygon_count_range: JtCountRange,
    pub untransformed_bbox: BBoxF32,
}

impl JtObjectTypeID for JtPartitionNodeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd103e,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtPartitionNodeElement {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        result.group_node_data = JtGroupNodeData::read(reader)?;
        result.partition_flags = reader.read_i32()?;
        result.file_name = reader.read_mb_string()?;

        if (result.partition_flags & 0x00000001) == 0 {
            result.transformed_bbox = reader.read_bboxf32()?;
        } else {
            // reserved field
            result.transformed_bbox = reader.read_bboxf32()?;
        }

        result.area = reader.read_f32()?;

        result.vertex_count_range = JtCountRange::read(reader)?;
        result.node_count_range = JtCountRange::read(reader)?;
        result.polygon_count_range = JtCountRange::read(reader)?;

        if (result.partition_flags & 0x00000001) != 0 {
            result.untransformed_bbox = reader.read_bboxf32()?;
        }

        Ok(result)
    }
}
