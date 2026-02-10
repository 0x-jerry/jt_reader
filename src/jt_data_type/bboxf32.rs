use anyhow::Result;

use crate::{jt_data::JtData, jt_data_type::vec3::CoordF32, jt_reader::JtReader};

#[derive(Debug, Default)]
pub struct BBoxF32 {
    pub min: CoordF32,
    pub max: CoordF32,
}

pub type DirF32 = CoordF32;

impl JtData for BBoxF32 {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let min = CoordF32::read(reader)?;
        let max = CoordF32::read(reader)?;

        Ok(Self { min, max })
    }
}
