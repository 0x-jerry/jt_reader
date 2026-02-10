use anyhow::Result;

use crate::{
    jt_data::{
        JtData, jt_topo_mesh_lod_data::JtTopoMeshLODData,
        jt_topologically_compressed_rep_data::JtTopologicallyCompressedRepData,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtTopoMeshTopologicallyCompressedLODData {
    pub mesh_lod_data: JtTopoMeshLODData,
    pub version: i16,
    pub data: JtTopologicallyCompressedRepData,
}

impl JtData for JtTopoMeshTopologicallyCompressedLODData {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mesh_lod_data = JtTopoMeshLODData::read(reader)?;
        let version = reader.read_i16()?;
        println!("JtTopoMeshTopologicallyCompressedLODData version {}", version);

        let data = JtTopologicallyCompressedRepData::read(reader)?;

        Ok(Self {
            mesh_lod_data,
            version,
            data,
        })
    }
}
