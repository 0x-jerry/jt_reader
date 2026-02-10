use anyhow::{Ok, Result, bail};

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug, Default)]
pub struct JtVec<T> {
    pub count: i32,
    pub data: Vec<T>,
}

impl JtVec<i32> {
    /// Compressed Data Packet
    pub fn read_by_cdp(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();
        let codec_type = reader.read_u8()?;

        println!("codec_type: {codec_type}");

        // Null Codec
        if codec_type == 0 {
            let length = reader.read_i32()?;
            println!("length: {length}");

            bail!("Null Codec")
        }

        // Arithmetic CODEC
        if codec_type == 3 {
            // todo: read I32 Probability Contexts
            let int32_probability_contexts_data = {
                let probability_context_table_count = reader.read_u8()?;

                println!(
                    "probability_context_table_count: {}",
                    probability_context_table_count
                );
            };

            let out_of_band_value_count = reader.read_i32()?;

            if out_of_band_value_count > 0 {
                // todo, read Int32 Compressed Data Packet
            }
        }

        // not equal Null CODEC
        if codec_type != 0 {
            let code_text_length = reader.read_i32()?;
            let value_element_count = reader.read_i32()?;

            let value_element_count = reader.read_i32()?;

            // todo
            let probability_context_table_count = 0;
            if probability_context_table_count > 1 {
                let symbol_count = reader.read_i32()?;
            }
        }

        // let code_text = reader.read_jt_vec_u32()?;

        bail!("test");

        Ok(result)
    }
}

macro_rules! impl_jt_data {
    ($type:ty, $reader_fn_name:ident) => {
        impl JtData for JtVec<$type> {
            fn read(reader: &mut JtReader) -> Result<Self> {
                let count = reader.read_i32()?;
                let data = reader.$reader_fn_name(count as usize)?;

                Ok(Self { count, data })
            }
        }
    };
}

impl_jt_data!(i32, read_i32_array);
impl_jt_data!(u32, read_u32_array);
impl_jt_data!(f32, read_f32_array);
impl_jt_data!(f64, read_f64_array);

pub type JtVecI32 = JtVec<i32>;
pub type JtVecU32 = JtVec<u32>;
pub type JtVecF32 = JtVec<f32>;

pub type JtVecF64 = JtVec<f64>;
