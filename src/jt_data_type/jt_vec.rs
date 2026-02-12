use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug, Default)]
pub struct JtVec<T> {
    pub count: i32,
    pub data: Vec<T>,
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
