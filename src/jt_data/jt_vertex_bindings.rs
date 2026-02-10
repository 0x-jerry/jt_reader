use anyhow::Result;

use crate::{jt_data::JtData, jt_reader::JtReader};

#[derive(Debug, Default)]
pub struct JtVertexBindings(u64);

macro_rules! match_bit {
    ($val:expr, $( ($bit:expr, $ret:expr) ),*) => {
        match $val {
            $(
                v if (v & (1 << $bit)) != 0 => $ret,
            )*
            _ => 0,
        }
    };
}

impl JtVertexBindings {
    pub fn vertex_coord_components(&self) -> u8 {
        let v = match_bit! {
            self.0,
            (0, 2),
            (1, 3),
            (2, 4)
        };

        v.into()
    }

    pub fn is_normal_binding(&self) -> bool {
        let v = match_bit! {
            self.0,
            (3, 1)
        };

        v == 1
    }

    pub fn color_components(&self) -> u8 {
        let v = match_bit! {
            self.0,
            (4, 3),
            (5, 4)
        };

        v.into()
    }

    pub fn is_vertex_flag_binding(&self) -> bool {
        let v = match_bit! {
            self.0,
            (6, 1)
        };

        v == 1
    }

    pub fn texture_coord_component(&self, texture_coord_index: u8) -> u8 {
        let start_bit = 8 + texture_coord_index * 4;

        let v = match_bit! {
            self.0,
            (start_bit, 1),
            (start_bit + 1, 2),
            (start_bit + 2, 3),
            (start_bit + 3, 4)
        };

        v.into()
    }

    pub fn is_auxiliary_vertex_field_binding(&self) -> bool {
        let v = match_bit! {
            self.0,
            (63, 1)
        };

        v == 1
    }
}

impl JtData for JtVertexBindings {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let vertex_bindings = reader.read_u64()?;
        Ok(Self(vertex_bindings))
    }
}
