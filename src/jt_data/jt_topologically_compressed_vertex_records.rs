use anyhow::Result;

use crate::{
    jt_data::{
        JtData, jt_compressed_vertex_color_array::JtCompressedVertexColorArray,
        jt_compressed_vertex_coordinate_array::JtCompressedVertexCoordinateArray,
        jt_compressed_vertex_flag_array::JtCompressedVertexFlagArray,
        jt_compressed_vertex_normal_array::JtCompressedVertexNormalArray,
        jt_compressed_vertex_texture_coordinate_array::JtCompressedVertexTextureCoordinateArray,
        jt_quantization_parameters::JtQuantizationParameters, jt_vertex_bindings::JtVertexBindings,
    },
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtTopologicallyCompressedVertexRecords {
    pub vertex_bindings: JtVertexBindings,
    pub quantization_parameters: JtQuantizationParameters,
    pub number_of_topological_vertices: i32,
    pub number_of_vertex_attributes: i32,

    pub vertex_coordinate_array: Option<JtCompressedVertexCoordinateArray>,
    pub vertex_normal_array: Option<JtCompressedVertexNormalArray>,
    pub vertex_color_array: Option<JtCompressedVertexColorArray>,
    pub vertex_texture_coordinate_array: [Option<JtCompressedVertexTextureCoordinateArray>; 8],
    pub vertex_flag_array: Option<JtCompressedVertexFlagArray>,
}

impl JtData for JtTopologicallyCompressedVertexRecords {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();
        result.vertex_bindings = JtVertexBindings::read(reader)?;
        result.quantization_parameters = JtQuantizationParameters::read(reader)?;
        result.number_of_topological_vertices = reader.read_i32()?;

        if result.number_of_topological_vertices < 0 {
            return Ok(result);
        }

        result.number_of_vertex_attributes = reader.read_i32()?;

        if result.vertex_bindings.vertex_coord_components() > 0 {
            result.vertex_coordinate_array = Some(JtCompressedVertexCoordinateArray::read(reader)?);
        }

        if result.vertex_bindings.is_normal_binding() {
            result.vertex_normal_array = Some(JtCompressedVertexNormalArray::read(reader)?);
        }

        if result.vertex_bindings.color_components() > 0 {
            result.vertex_color_array = Some(JtCompressedVertexColorArray::read(reader)?);
        }

        {
            let mut vertex_texture_coordinate_array = Vec::new();
            for n in 0..8 {
                if result.vertex_bindings.texture_coord_component(n) > 0 {
                    vertex_texture_coordinate_array.push(Some(
                        JtCompressedVertexTextureCoordinateArray::read(reader)?,
                    ));
                } else {
                    vertex_texture_coordinate_array.push(None);
                }
            }

            result.vertex_texture_coordinate_array = vertex_texture_coordinate_array
                .try_into()
                .expect("vertex_texture_coordinate_array: Convert vec to slice failed");
        }

        if result.vertex_bindings.is_vertex_flag_binding() {
            result.vertex_flag_array = Some(JtCompressedVertexFlagArray::read(reader)?);
        }

        Ok(result)
    }
}
