use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug, Default)]
pub struct Vec3<T> {
    pub data: [T; 3],
}

impl<T> Vec3<T>
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

    pub fn r(&self) -> T {
        return self.data[0];
    }

    pub fn g(&self) -> T {
        return self.data[1];
    }

    pub fn b(&self) -> T {
        return self.data[2];
    }
}

macro_rules! impl_jt_data {
    ($type:ty, $reader_fn_name:ident) => {
        impl JtData for Vec3<$type> {
            fn read(reader: &mut JtReader) -> Result<Self> {
                let data = reader.$reader_fn_name(3)?;

                Ok(Self {
                    data: data.try_into().unwrap(),
                })
            }
        }
    };
}

impl_jt_data!(f32, read_f32_array);
impl_jt_data!(f64, read_f64_array);

pub type CoordF32 = Vec3<f32>;
pub type CoordF64 = Vec3<f64>;

pub type RGB = Vec3<f32>;
