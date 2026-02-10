use anyhow::Result;
use uuid::Uuid;

use crate::{
    jt_data::{
        JtData, JtObjectTypeID, jt_common_marker::END_OF_ELEMENTS_MARKER,
        jt_partition_node_element::JtPartitionNodeElement,
    },
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub enum JtGraphElement {
    #[default]
    None,
    EndOfElements,
    BaseNode,
    PartitionNode(JtPartitionNodeElement),
    GroupNode,
    InstanceNode,
    PartNode,
    MetaDataNode,
    LODNode,
    RangeLODNode,
    SwitchNode,
    ShapeNode,
}

impl JtGraphElement {
    pub fn read(reader: &mut JtReader, object_type_id: Uuid) -> Result<Self> {
        match object_type_id {
            JtPartitionNodeElement::OBJECT_TYPE_ID => {
                let partition_node_element = JtPartitionNodeElement::read(reader)?;
                Ok(Self::PartitionNode(partition_node_element))
            }
            END_OF_ELEMENTS_MARKER => Ok(Self::EndOfElements),
            _ => Ok(Self::None),
        }
    }

    pub fn is_end(&self) -> bool {
        if let JtGraphElement::EndOfElements = self {
            true
        } else {
            false
        }
    }

    pub fn is_none(&self) -> bool {
        if let JtGraphElement::None = self {
            true
        } else {
            false
        }
    }
}
