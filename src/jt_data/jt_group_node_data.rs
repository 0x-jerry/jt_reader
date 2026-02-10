use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_base_node_data::JtBaseNodeData},
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtGroupNodeData {
    pub base_node_data: JtBaseNodeData,
    pub version: i16,
    pub child_count: i32,
    pub child_node_object_id: Vec<i32>,
}

impl JtData for JtGroupNodeData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let base_node_data = JtBaseNodeData::read(reader)?;
        let version = reader.read_i16()?;
        let child_count = reader.read_i32()?;
        let child_node_object_id = reader.read_i32_array(child_count as usize)?;

        Ok(Self {
            base_node_data,
            version,
            child_count,
            child_node_object_id,
        })
    }
}
