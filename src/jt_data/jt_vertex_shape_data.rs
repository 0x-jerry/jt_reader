use crate::jt_data::{
    JtData, jt_base_shape_data::JtBaseShapeData,
    jt_quantization_parameters::JtQuantizationParameters,
};

#[derive(Debug, Default)]
pub struct JtVertexShapeData {
    pub base_shape_data: JtBaseShapeData,
    pub version: i16,
    pub vertex_binding: u64,
    pub quantization_parameters: JtQuantizationParameters,
}

impl JtData for JtVertexShapeData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();

        result.base_shape_data = JtBaseShapeData::read(reader)?;
        result.version = reader.read_i16()?;
        result.vertex_binding = reader.read_u64()?;
        result.quantization_parameters = JtQuantizationParameters::read(reader)?;

        if result.version != 1 {
            result.vertex_binding = reader.read_u64()?;
        }

        Ok(result)
    }
}
