use crate::jt_data::JtData;

#[derive(Debug, Default)]
pub struct JtBaseAttributeData {
    pub version: i16,
    pub state_flags: u8,
    pub field_inhibit_flags: u32,
}

impl JtData for JtBaseAttributeData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            version: reader.read_i16()?,
            state_flags: reader.read_u8()?,
            field_inhibit_flags: reader.read_u32()?,
        })
    }
}
