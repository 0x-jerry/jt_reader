use crate::jt_reader::JtReader;
use anyhow::Result;

macro_rules! impl_jt_read_array_fn {
    ($type:ty, $reader_fn_name:ident, $read_name:ident) => {
        impl JtReader {
            pub fn $reader_fn_name(&mut self, count: usize) -> Result<Vec<$type>> {
                let mut arr = Vec::with_capacity(count);

                for _ in 0..count {
                    arr.push(self.$read_name()?);
                }

                Ok(arr)
            }
        }
    };
}

impl_jt_read_array_fn!(i32, read_i32_array, read_i32);
impl_jt_read_array_fn!(u32, read_u32_array, read_u32);
impl_jt_read_array_fn!(f32, read_f32_array, read_f32);

impl_jt_read_array_fn!(i64, read_i64_array, read_i64);
impl_jt_read_array_fn!(u64, read_u64_array, read_u64);
impl_jt_read_array_fn!(f64, read_f64_array, read_f64);
