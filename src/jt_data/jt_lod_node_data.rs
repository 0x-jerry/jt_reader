use crate::{
    jt_data::{JtData, jt_group_node_data::JtGroupNodeData},
    jt_data_type::jt_vec::JtVecF32,
};

#[derive(Debug, Default)]
pub struct JtLODNodeData {
    pub group_node_data: JtGroupNodeData,
    pub version: i16,
    pub reserved_field: JtVecF32,
    pub reserved_field_2: i32,
}

impl JtData for JtLODNodeData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self>
    {
        Ok(Self{
            group_node_data: JtGroupNodeData::read(reader)?,
            version: reader.read_i16()?,
            reserved_field: reader.read_jt_vec_f32()?,
            reserved_field_2: reader.read_i32()?,
        })
    }
}
