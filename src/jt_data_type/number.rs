use crate::jt_reader::{ByteOrder, JtReader};
use anyhow::Result;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

macro_rules! impl_jt_read_fn {
    ($type:ty, $reader_fn_name:ident) => {
        impl JtReader {
            pub fn $reader_fn_name(&mut self) -> Result<$type> {
                match self.byte_order {
                    ByteOrder::LittleEndian => self
                        .reader
                        .$reader_fn_name::<LittleEndian>()
                        .map_err(Into::into),
                    ByteOrder::BigEndian => self
                        .reader
                        .$reader_fn_name::<BigEndian>()
                        .map_err(Into::into),
                }
            }
        }
    };
}

impl_jt_read_fn!(u16, read_u16);
impl_jt_read_fn!(i16, read_i16);

impl_jt_read_fn!(u32, read_u32);
impl_jt_read_fn!(i32, read_i32);
impl_jt_read_fn!(f32, read_f32);

impl_jt_read_fn!(u64, read_u64);
impl_jt_read_fn!(i64, read_i64);
impl_jt_read_fn!(f64, read_f64);
