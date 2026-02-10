use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_topo_mesh_lod_data::JtTopoMeshLODData},
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtTopoMeshCompressedLODData {
    pub mesh_lod_data: JtTopoMeshLODData,
    pub version: i16,
}

impl JtData for JtTopoMeshCompressedLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let topo_mesh_lod_data = JtTopoMeshLODData::read(reader)?;

        let version = reader.read_i16()?;

        // todo
        if version >= 2 {
            // v2
        } else {
            // v1
        }

        Ok(Self {
            mesh_lod_data: topo_mesh_lod_data,
            version,
        })
    }
}
