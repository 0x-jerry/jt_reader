use anyhow::Result;

use crate::{
    jt_data::JtData,
    jt_decode::{jt_codec_type::JtCodecType, jt_probability_context_v2::Int32ProbabilityContextV2},
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtCDP2Data {
    pub count: i32,
    pub data: Vec<u32>,
}

impl JtCDP2Data {
    /// Compressed Data Packet V2
    pub fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();
        result.count = reader.read_i32()?;

        if result.count == 0 {
            return Ok(result);
        }

        let codec_type = JtCodecType::from(reader.read_u8()?)?;

        log::debug!("codec_type: {:?}", codec_type);

        if codec_type == JtCodecType::Chopper {
            let chop_bits = reader.read_u8()?;

            if chop_bits == 0 && codec_type == JtCodecType::Bitlength {
                return Self::read(reader);
            }

            log::info!("chop_bits: {}", chop_bits);

            let value_bias = reader.read_i32()?;

            let value_span_bits = reader.read_u8()?;

            let chopped_msb_data = Self::read(reader)?;
            let chopped_lsb_data = Self::read(reader)?;

            return Ok(result);
        }

        let code_text_length = reader.read_i32()?;

        if code_text_length <= 0 {
            return Ok(result);
        }

        let count = (code_text_length + 31) / 32;
        let code_text_word: Vec<u32> = reader.read_u32_array(count as usize)?;
        result.data = code_text_word;

        if codec_type == JtCodecType::Null {
            return Ok(result);
        }

        if codec_type == JtCodecType::Bitlength {
            // bail!("Bitlength CODEC");
        } else if codec_type == JtCodecType::Arithmetic {
            let int32_probability_contexts_data = Int32ProbabilityContextV2::read(reader)?;

            // bail!("Arithmetic CODEC");
            let oob_data_values = Self::read(reader)?;
        }

        Ok(result)
    }
}
