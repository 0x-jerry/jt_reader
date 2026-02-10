use crate::jt_data::{JtData, jt_uniform_quantizer_data::JtUniformQuantizerData};

#[derive(Debug, Default)]
pub struct JtColorQuantizerData {
    pub hsv_flag: u8,
    pub red_uniform_quantizer_data: JtUniformQuantizerData,
    pub green_uniform_quantizer_data: JtUniformQuantizerData,
    pub blue_uniform_quantizer_data: JtUniformQuantizerData,
    pub alpha_uniform_quantizer_data: JtUniformQuantizerData,

    pub numer_of_hue_bits: u8,
    pub numer_of_saturation_bits: u8,
    pub numer_of_value_bits: u8,
    pub numer_of_alpha_bits: u8,
}

impl JtData for JtColorQuantizerData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();

        result.hsv_flag = reader.read_u8()?;

        if result.hsv_flag == 1 {
            result.numer_of_hue_bits = reader.read_u8()?;
            result.numer_of_saturation_bits = reader.read_u8()?;
            result.numer_of_value_bits = reader.read_u8()?;
            result.numer_of_alpha_bits = reader.read_u8()?;
        } else {
            result.red_uniform_quantizer_data = JtUniformQuantizerData::read(reader)?;
            result.green_uniform_quantizer_data = JtUniformQuantizerData::read(reader)?;
            result.blue_uniform_quantizer_data = JtUniformQuantizerData::read(reader)?;
            result.alpha_uniform_quantizer_data = JtUniformQuantizerData::read(reader)?;
        }

        Ok(result)
    }
}
