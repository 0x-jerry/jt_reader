use anyhow::Result;

use crate::{
    jt_data::JtData,
    jt_data_type::{
        bboxf32::BBoxF32,
        jt_string::JtString,
        jt_vec::{JtVecI32, JtVecF32, JtVecU32},
        matrix::Mx4F32,
        mbstring::MbString,
        vec3::{CoordF32, CoordF64, RGB},
        vec4::{HCoordF32, HCoordF64, PlainF32, Quaternion, RGBA},
    },
    jt_reader::JtReader,
};

macro_rules! impl_jt_read_fn_macro {
    ($type:ty, $reader_fn_name:ident) => {
        impl JtReader {
            pub fn $reader_fn_name(&mut self) -> Result<$type> {
                <$type>::read(self)
            }
        }
    };
}

impl_jt_read_fn_macro!(BBoxF32, read_bboxf32);
impl_jt_read_fn_macro!(JtString, read_jt_string);
impl_jt_read_fn_macro!(MbString, read_mb_string);

impl_jt_read_fn_macro!(JtVecI32, read_jt_vec_i32);
impl_jt_read_fn_macro!(JtVecF32, read_jt_vec_f32);
impl_jt_read_fn_macro!(JtVecU32, read_jt_vec_u32);
impl_jt_read_fn_macro!(Mx4F32, read_mx4f32);

impl_jt_read_fn_macro!(CoordF32, read_coordf32);
impl_jt_read_fn_macro!(CoordF64, read_coordf64);
impl_jt_read_fn_macro!(RGB, read_rgb);

impl_jt_read_fn_macro!(HCoordF32, read_hcoordf32);
impl_jt_read_fn_macro!(HCoordF64, read_hcoordf64);
impl_jt_read_fn_macro!(PlainF32, read_plainf32);
impl_jt_read_fn_macro!(Quaternion, read_quaternion);
impl_jt_read_fn_macro!(RGBA, read_rgba);
