use anyhow::{Result, bail};

use crate::{
    jt_data::{
        JtData, jt_base_shape_lod_data::JtBaseShapeLODData,
        jt_topo_mesh_topologically_compressed_lod_data::JtTopoMeshTopologicallyCompressedLODData,
        jt_vertex_bindings::JtVertexBindings,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtVertexShapeLODData {
    pub version: i16,
    pub vertex_bindings: JtVertexBindings,
}

impl JtData for JtVertexShapeLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        // https://github.com/cbsghost/oce-jt/blob/a7ea581f0dba325c649bab7fe1bc68ca5e85b946/TKJT/src/JtElement/JtElement_ShapeLOD_Vertex.cxx#L262
        let _base = JtBaseShapeLODData::read(reader)?;

        let version = reader.read_i16()?;
        if version != 1 {
            bail!("Invalid JtVertexShapeLODData version {}", version);
        }

        let vertex_bindings = JtVertexBindings::read(reader)?;

        // todo
        let is_topo_mesh_element = true;

        if is_topo_mesh_element {
            JtTopoMeshTopologicallyCompressedLODData::read(reader)?;
        }

        Ok(Self {
            version,
            vertex_bindings,
        })
    }
}
