use crate::jt_reader::JtReader;
use anyhow::Result;
use uuid::Uuid;

pub mod jt_base_shape_lod_data;
pub mod jt_base_type;
pub mod jt_color_quantizer_data;
pub mod jt_compressed_vertex_color_array;
pub mod jt_compressed_vertex_coordinate_array;
pub mod jt_compressed_vertex_flag_array;
pub mod jt_compressed_vertex_normal_array;
pub mod jt_compressed_vertex_texture_coordinate_array;
pub mod jt_element_header;
pub mod jt_file_header;
pub mod jt_group_node_data;
pub mod jt_logic_element_header;
pub mod jt_partition_node_element;
pub mod jt_point_quantizer_data;
pub mod jt_base_node_data;
pub mod jt_lsg_segment;
pub mod jt_graph_element;
pub mod jt_common_marker;
pub mod jt_quantization_parameters;
pub mod jt_segment;
pub mod jt_segment_data;
pub mod jt_segment_header;
pub mod jt_segment_type;
pub mod jt_shape;
pub mod jt_texture_quantizer_data;
pub mod jt_toc_entity;
pub mod jt_topo_mesh_compressed_lod_data;
pub mod jt_topo_mesh_lod_data;
pub mod jt_topo_mesh_topologically_compressed_lod_data;
pub mod jt_topologically_compressed_rep_data;
pub mod jt_topologically_compressed_vertex_records;
pub mod jt_uniform_quantizer_data;
pub mod jt_vertex_bindings;
pub mod jt_vertex_shape_lod_data;

pub trait JtData {
    fn read(reader: &mut JtReader) -> Result<Self>
    where
        Self: Sized;
}

pub trait JtObjectTypeID {
    const OBJECT_TYPE_ID: Uuid;
}

pub trait JtCompressedSegment {
    const IS_COMPRESSED: bool;
}
