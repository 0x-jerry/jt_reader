use anyhow::{Result, bail};
use uuid::Uuid;

use crate::{
    jt_data::{
        JtData, JtObjectTypeID, jt_element_header::JtElementHeader,
        jt_floating_point_property_atom_element::JtFloatingPointPropertyAtomElement,
        jt_geometric_transform_attribute_element::JtGeometricTransformAttributeElement,
        jt_group_node_element::JtGroupNodeElement, jt_instance_node_element::JtInstanceNodeElement,
        jt_late_loaded_property_atom_element::JtLateLoadedPropertyAtomElement,
        jt_material_attribute_element::JtMaterialAttributeElement,
        jt_meta_data_node_element::JtMetaDataNodeElement, jt_part_node_element::JtPartNodeElement,
        jt_partition_node_element::JtPartitionNodeElement,
        jt_property_proxy_meta_data_element::JtPropertyProxyMetaDataElement,
        jt_range_lod_node_element::JtRangeLODNodeElement,
        jt_string_property_atom_element::JtStringPropertyAtomElement,
        jt_tri_strip_set_shape_lod_element::JtTriStripSetShapeLODElement,
        jt_tri_strip_set_shape_node_element::JtTriStripSetShapeNodeElement,
    },
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub enum JtElementValue {
    #[default]
    None,
    // Nodes
    BaseNode,
    PartitionNode(JtPartitionNodeElement),
    GroupNode(JtGroupNodeElement),
    InstanceNode(JtInstanceNodeElement),
    PartNode(JtPartNodeElement),
    MetaDataNode(JtMetaDataNodeElement),
    LODNode,
    RangeLODNode(JtRangeLODNodeElement),
    SwitchNode,
    TriStripSetShapeNode(JtTriStripSetShapeNodeElement),

    // Attributes
    GeometricTransformAttribute(JtGeometricTransformAttributeElement),
    MaterialAttribute(JtMaterialAttributeElement),

    // Properties
    BasePropertyAtom,
    StringPropertyAtom(JtStringPropertyAtomElement),
    LateLoadedPropertyAtom(JtLateLoadedPropertyAtomElement),
    FloatingPointPropertyAtom(JtFloatingPointPropertyAtomElement),

    // Other
    TriStripSetShapeLOD(JtTriStripSetShapeLODElement),

    // Meta data
    PropertyProxyMetaData(JtPropertyProxyMetaDataElement),
}

impl JtElementValue {
    pub fn read(reader: &mut JtReader, object_type_id: Uuid) -> Result<Self> {
        let result = match object_type_id {
            // Nodes
            JtPartitionNodeElement::OBJECT_TYPE_ID => {
                let element = JtPartitionNodeElement::read(reader)?;
                Self::PartitionNode(element)
            }
            JtMetaDataNodeElement::OBJECT_TYPE_ID => {
                let element = JtMetaDataNodeElement::read(reader)?;
                Self::MetaDataNode(element)
            }
            JtInstanceNodeElement::OBJECT_TYPE_ID => {
                let element = JtInstanceNodeElement::read(reader)?;
                Self::InstanceNode(element)
            }
            JtPartNodeElement::OBJECT_TYPE_ID => {
                let element = JtPartNodeElement::read(reader)?;
                Self::PartNode(element)
            }
            JtRangeLODNodeElement::OBJECT_TYPE_ID => {
                let element = JtRangeLODNodeElement::read(reader)?;
                Self::RangeLODNode(element)
            }
            JtGroupNodeElement::OBJECT_TYPE_ID => {
                let element = JtGroupNodeElement::read(reader)?;
                Self::GroupNode(element)
            }
            JtTriStripSetShapeNodeElement::OBJECT_TYPE_ID => {
                let element = JtTriStripSetShapeNodeElement::read(reader)?;
                Self::TriStripSetShapeNode(element)
            }

            // Attributes
            JtGeometricTransformAttributeElement::OBJECT_TYPE_ID => {
                let element = JtGeometricTransformAttributeElement::read(reader)?;
                Self::GeometricTransformAttribute(element)
            }
            JtMaterialAttributeElement::OBJECT_TYPE_ID => {
                let element = JtMaterialAttributeElement::read(reader)?;
                Self::MaterialAttribute(element)
            }

            // Properties
            JtStringPropertyAtomElement::OBJECT_TYPE_ID => {
                let element = JtStringPropertyAtomElement::read(reader)?;
                Self::StringPropertyAtom(element)
            }
            JtLateLoadedPropertyAtomElement::OBJECT_TYPE_ID => {
                let element = JtLateLoadedPropertyAtomElement::read(reader)?;
                Self::LateLoadedPropertyAtom(element)
            }
            JtFloatingPointPropertyAtomElement::OBJECT_TYPE_ID => {
                let element = JtFloatingPointPropertyAtomElement::read(reader)?;
                Self::FloatingPointPropertyAtom(element)
            }

            // Other
            JtTriStripSetShapeLODElement::OBJECT_TYPE_ID => {
                let element = JtTriStripSetShapeLODElement::read(reader)?;
                Self::TriStripSetShapeLOD(element)
            }

            // Meta data
            JtPropertyProxyMetaDataElement::OBJECT_TYPE_ID => {
                let element = JtPropertyProxyMetaDataElement::read(reader)?;
                Self::PropertyProxyMetaData(element)
            }
            _ => Self::None,
        };

        Ok(result)
    }

    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct JtElement {
    pub length: i32,
    pub header: JtElementHeader,
    pub value: JtElementValue,
}

impl JtElement {
    pub fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        result.length = reader.read_i32()?;

        result.header = JtElementHeader::read(reader)?;

        if result.is_end_marker_element() {
            return Ok(result);
        }

        log::debug!("reading element: {:?}", result.header);
        result.value = JtElementValue::read(reader, result.header.object_type_id)?;

        if result.value.is_none() {
            bail!("Unimplemented object type: {:?}", result.header);
        }

        Ok(result)
    }

    pub fn is_end_marker_element(&self) -> bool {
        self.header.is_end_marker_object_type()
    }
}
