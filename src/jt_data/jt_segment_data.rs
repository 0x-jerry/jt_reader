use anyhow::{Ok, Result};

use crate::{
    jt_data::{
        JtData, jt_base_type::JtBaseType, jt_element_header::JtElementHeader,
        jt_logic_element_header::JtLogicElementHeader,
        jt_partition_node_element::JtPartitionNodeElement, jt_shape::JtShape,
    },
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub enum JtSegmentData {
    #[default]
    None,
    Shape(JtShape),
}

impl JtSegmentData {
    pub fn read(reader: &mut JtReader, is_zlib_applyied: bool) -> Result<Self> {
        if let Some(r) = Self::read_optional(reader, is_zlib_applyied)? {
            return Ok(r);
        }

        Ok(JtSegmentData::None)
    }

    pub fn read_optional(reader: &mut JtReader, is_zlib_applied: bool) -> Result<Option<Self>> {
        let mut logic_header = JtLogicElementHeader::read(reader, is_zlib_applied)?;

        let reader = if logic_header.is_zlib_compressed() {
            &mut reader.inflate(logic_header.compression_data_length as usize)?
        } else {
            reader
        };

        if logic_header.is_zlib_compressed() {
            logic_header.read_length(reader)?;
        }

        println!("logic element header: {:?}", logic_header);

        let element_header = JtElementHeader::read(reader)?;

        println!("element header: {:?}", element_header);
        match element_header.object_base_type {
            JtBaseType::ShapeLOD => {
                println!(
                    "Shape type: {:?}",
                    JtShape::shape_type(element_header.object_type_id)
                );

                let shape = JtShape::read_by_object_type_id(reader, element_header.object_type_id)?;

                return Ok(shape.map(Self::Shape));
            }
            JtBaseType::GroupGraphNode => {
                let node = JtPartitionNodeElement::read(reader)?;

                println!("node: {:#?}", node);
            }
            _ => {}
        }

        Ok(None)
    }
}
