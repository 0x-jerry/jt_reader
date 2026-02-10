use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

pub struct Mx4<T> {
    pub data: [T; 16],
}

macro_rules! impl_jt_data {
    ($type:ty, $reader_fn_name:ident) => {
        impl JtData for Mx4<$type> {
            fn read(reader: &mut JtReader) -> Result<Self> {
                let data = reader.$reader_fn_name(16)?;

                Ok(Self {
                    data: data.try_into().unwrap(),
                })
            }
        }
    };
}

impl_jt_data!(f32, read_f32_array);
impl_jt_data!(f64, read_f64_array);

pub type Mx4F32 = Mx4<f32>;
