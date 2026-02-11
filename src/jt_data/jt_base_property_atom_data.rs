use crate::jt_data::JtData;

#[derive(Debug, Default)]
pub struct JtBasePropertyAtomData {
    pub version: i16,
    pub state_flags: u32,
}

impl JtData for JtBasePropertyAtomData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        Ok(Self {
            version: reader.read_i16()?,
            state_flags: reader.read_u32()?,
        })
    }
}
