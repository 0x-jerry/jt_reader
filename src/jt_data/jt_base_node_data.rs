use crate::jt_data::JtData;

#[derive(Debug, Default)]
pub struct JtBaseNodeData {
    pub version: i16,
    pub node_flags: u32,
    pub attribute_count: i32,
    pub attribute_object_id: Vec<i32>,
}

impl JtData for JtBaseNodeData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let version = reader.read_i16()?;
        let node_flags = reader.read_u32()?;
        let attribute_count = reader.read_i32()?;
        let attribute_object_id = reader.read_i32_array(attribute_count as usize)?;

        Ok(Self {
            version,
            node_flags,
            attribute_count,
            attribute_object_id,
        })
    }
}
