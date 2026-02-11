use crate::jt_data::JtData;

#[derive(Debug, Default)]
pub struct JtCountRange {
    pub min: i32,
    pub max: i32,
}

impl JtData for JtCountRange {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            min: reader.read_i32()?,
            max: reader.read_i32()?,
        })
    }
}
