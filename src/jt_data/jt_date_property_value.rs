use crate::jt_data::JtData;

#[derive(Debug, Default)]
pub struct JtDatePropertyValue {
    pub year: i16,
    pub month: i16,
    pub day: i16,
    pub hour: i16,
    pub minute: i16,
    pub second: i16,
}

impl JtData for JtDatePropertyValue {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result = Self::default();

        result.year = reader.read_i16()?;
        result.month = reader.read_i16()?;
        result.day = reader.read_i16()?;
        result.hour = reader.read_i16()?;
        result.minute = reader.read_i16()?;
        result.second = reader.read_i16()?;

        Ok(result)
    }
}
