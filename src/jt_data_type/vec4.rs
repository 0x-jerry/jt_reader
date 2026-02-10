use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug)]
pub struct Vec4<T> {
    pub data: [T; 4],
}

impl<T> Vec4<T>
where
    T: Copy,
{
    pub fn x(&self) -> T {
        return self.data[0];
    }

    pub fn y(&self) -> T {
        return self.data[1];
    }

    pub fn z(&self) -> T {
        return self.data[2];
    }

    pub fn w(&self) -> T {
        return self.data[3];
    }

    pub fn r(&self) -> T {
        return self.data[0];
    }

    pub fn g(&self) -> T {
        return self.data[1];
    }

    pub fn b(&self) -> T {
        return self.data[2];
    }

    pub fn a(&self) -> T {
        return self.data[3];
    }
}

macro_rules! impl_jt_data {
    ($type:ty, $reader_fn_name:ident) => {
        impl JtData for Vec4<$type> {
            fn read(reader: &mut JtReader) -> Result<Self> {
                let data = reader.$reader_fn_name(4)?;

                Ok(Self {
                    data: data.try_into().unwrap(),
                })
            }
        }
    };
}

impl_jt_data!(f32, read_f32_array);
impl_jt_data!(f64, read_f64_array);

pub type HCoordF32 = Vec4<f32>;
pub type HCoordF64 = Vec4<f64>;

pub type PlainF32 = Vec4<f32>;

pub type Quaternion = Vec4<f32>;

pub type RGBA = Vec4<f32>;
