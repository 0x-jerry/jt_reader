use mesh_tools::compat::{Point3, Vector3};

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Point3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub indices: Vec<u32>,
}
