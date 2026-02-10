use anyhow::Result;
use uuid::Uuid;

use crate::{
    jt_data::{
        JtData, JtObjectTypeID, jt_base_shape_lod_data::JtBaseShapeLODData,
        jt_logic_element_header::JtLogicElementHeader,
        jt_topo_mesh_compressed_lod_data::JtTopoMeshCompressedLODData,
        jt_topo_mesh_topologically_compressed_lod_data::JtTopoMeshTopologicallyCompressedLODData,
        jt_vertex_bindings::JtVertexBindings, jt_vertex_shape_lod_data::JtVertexShapeLODData,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub enum JtShape {
    Base(JtBaseShapeLOD),
    Vertex(JtVertexShapeLOD),
    TriStripSet(JtTriStripSetShapeLOD),
}

impl JtShape {
    pub fn read_by_object_type_id(
        reader: &mut JtReader,
        object_type_id: Uuid,
    ) -> Result<Option<Self>> {
        match object_type_id {
            JtBaseShapeLOD::OBJECT_TYPE_ID => Ok(Some(Self::Base(JtBaseShapeLOD::read(reader)?))),
            JtVertexShapeLOD::OBJECT_TYPE_ID => {
                Ok(Some(Self::Vertex(JtVertexShapeLOD::read(reader)?)))
            }
            JtTriStripSetShapeLOD::OBJECT_TYPE_ID => Ok(Some(Self::TriStripSet(
                JtTriStripSetShapeLOD::read(reader)?,
            ))),
            _ => Ok(None),
        }
    }

    pub fn shape_type(shape_type_id: Uuid) -> String {
        match shape_type_id {
            JtBaseShapeLOD::OBJECT_TYPE_ID => "BaseShapeLOD".to_string(),
            JtVertexShapeLOD::OBJECT_TYPE_ID => "VertexShapeLOD".to_string(),
            JtTriStripSetShapeLOD::OBJECT_TYPE_ID => "TriStripSetShapeLOD".to_string(),
            _ => "UnknownShapeLOD".to_string(),
        }
    }
}

impl JtData for JtShape {
    fn read(reader: &mut JtReader) -> Result<Self> {
        Ok(Self::Base(JtBaseShapeLOD::read(reader)?))
    }
}

#[derive(Debug)]
pub struct JtBaseShapeLOD {
    pub version: i16,
}

impl JtObjectTypeID for JtBaseShapeLOD {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd10a4,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtBaseShapeLOD {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let _logic_element_header = JtLogicElementHeader::read(reader, false)?;
        let version = JtBaseShapeLODData::read(reader)?.version;

        Ok(Self { version })
    }
}

#[derive(Debug)]
pub struct JtVertexShapeLOD {
    pub version: i16,
    pub vertex_bindings: JtVertexBindings,
    pub topo_mesh_lod: JtTopoMeshCompressedLODData,
}

impl JtObjectTypeID for JtVertexShapeLOD {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd10b0,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtVertexShapeLOD {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let _logic_element_header = JtLogicElementHeader::read(reader, false)?;
        let _base_shape_lod_data = JtBaseShapeLODData::read(reader)?;
        let vertex_shape_data = JtVertexShapeLODData::read(reader)?;

        let topo_mesh_lod = JtTopoMeshCompressedLODData::read(reader)?;

        Ok(Self {
            version: vertex_shape_data.version,
            vertex_bindings: vertex_shape_data.vertex_bindings,
            topo_mesh_lod,
        })
    }
}

#[derive(Debug)]
pub struct JtTriStripSetShapeLOD {
    pub version: i16,
    pub vertex_bindings: JtVertexBindings,
    pub topo_mesh_lod: JtTopoMeshTopologicallyCompressedLODData,
}

impl JtObjectTypeID for JtTriStripSetShapeLOD {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd10ab,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtTriStripSetShapeLOD {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let _logic_element_header = JtLogicElementHeader::read(reader, false)?;

        let vertex_shape_lod_data = JtVertexShapeLODData::read(reader)?;

        let topo_mesh_lod = JtTopoMeshTopologicallyCompressedLODData::read(reader)?;

        let version = reader.read_i16()?;

        Ok(Self {
            version,
            vertex_bindings: vertex_shape_lod_data.vertex_bindings,
            topo_mesh_lod,
        })
    }
}
