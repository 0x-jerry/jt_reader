use anyhow::Result;
use mesh_tools::{GltfBuilder, Triangle, compat::{Point3, Vector3}};
use crate::mesh::Mesh;

pub fn convert_to_glb(meshes: &[Mesh], output_path: &str) -> Result<()> {
    let mut builder = GltfBuilder::new();
    let mut nodes = Vec::new();

    if meshes.is_empty() {
        println!("No meshes provided, creating placeholder box.");
        let mesh_index = builder.create_box(1.0);
        let node = builder.add_node(Some("Placeholder".to_string()), Some(mesh_index), None, None, None);
        nodes.push(node);
    } else {
        for (i, mesh) in meshes.iter().enumerate() {
            println!("Processing mesh {}/{} (V: {}, I: {})", i+1, meshes.len(), mesh.vertices.len(), mesh.indices.len());
            
            if mesh.vertices.is_empty() { continue; }

            let triangles: Vec<Triangle> = mesh.indices.chunks(3)
                .filter(|c| c.len() == 3)
                .map(|c| Triangle { a: c[0], b: c[1], c: c[2] }) 
                .collect();

            let mesh_index = builder.create_custom_mesh(
                 Some(format!("Mesh_{}", i)),
                 &mesh.vertices,
                 &triangles,
                 if mesh.normals.is_empty() { None } else { Some(mesh.normals.clone()) },
                 None, // texcoords
                 None  // material
            );
            
            let node = builder.add_node(
                Some(format!("Node_{}", i)),
                Some(mesh_index),
                None,
                None,
                None,
            );
            nodes.push(node);
        }
    }

    builder.add_scene(Some("Main Scene".to_string()), Some(nodes));

    builder.export_glb(output_path).map_err(|e| anyhow::anyhow!("Failed to export GLB: {:?}", e))?;

    Ok(())
}
