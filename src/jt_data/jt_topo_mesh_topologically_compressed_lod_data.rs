use anyhow::{Result, bail};
use mesh_tools::mesh;

use crate::{
    jt_data::{
        JtData, jt_topo_mesh_lod_data::JtTopoMeshLODData,
        jt_topologically_compressed_rep_data::JtTopologicallyCompressedRepData,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtTopoMeshTopologicallyCompressedLODData {
    pub topo_mesh_lod_data: JtTopoMeshLODData,
    pub version: i16,
    pub data: JtTopologicallyCompressedRepData,
}

impl JtData for JtTopoMeshTopologicallyCompressedLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mesh_lod_data = JtTopoMeshLODData::read(reader)?;
        let version = reader.read_i16()?;

        if version != 1 && version != 2 {
            bail!(
                "Invalid JtTopoMeshTopologicallyCompressedLODData version {}",
                version
            );
        }

        log::info!("mesh_lod_data: {:#?}", mesh_lod_data);

        let data = JtTopologicallyCompressedRepData::read(reader)?;

        Ok(Self {
            topo_mesh_lod_data: mesh_lod_data,
            version,
            data,
        })
    }
}
