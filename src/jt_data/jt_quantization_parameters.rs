use crate::jt_data::JtData;

#[derive(Debug, Default)]
pub struct JtQuantizationParameters {
    pub bits_per_vertex: u8,
    pub normal_bits_factory: u8,
    pub bits_per_texture_coord: u8,
    pub bits_per_color: u8,
}

impl JtData for JtQuantizationParameters {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        return Ok(Self {
            bits_per_vertex: reader.read_u8()?,
            normal_bits_factory: reader.read_u8()?,
            bits_per_texture_coord: reader.read_u8()?,
            bits_per_color: reader.read_u8()?,
        });
    }
}
