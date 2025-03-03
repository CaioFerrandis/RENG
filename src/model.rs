use glam::{vec2, vec3, Vec4};
use assimp::{Importer, Vector3D};

use crate::{mesh::Mesh, Shader, Vertex};

pub fn load_model(path: &str, texture: u32) -> Vec<Mesh> {
    let mut meshes = Vec::new();

    let mut importer = Importer::new();
    importer.triangulate(true);
    importer.optimize_meshes(true);

    let scene = importer.read_file(path).expect("Failed to read file");

    for assimp_mesh in scene.mesh_iter() {
        let num_vertices = assimp_mesh.num_vertices();
        let indices: Vec<u32> = assimp_mesh
            .face_iter()
            .flat_map(|f| unsafe { std::slice::from_raw_parts(f.indices, f.num_indices as usize) })
            .copied()
            .collect();

        let mut vertices = Vec::with_capacity(num_vertices as usize);

        for i in 0..num_vertices {
            let v = assimp_mesh.get_vertex(i).unwrap();
            let pos = vec3(v.x, v.y, v.z);

            let v = assimp_mesh.get_normal(i).unwrap_or(Vector3D::new(0., 0., 0.));
            let normal = vec3(v.x, v.y, v.z);

            let v = assimp_mesh.get_texture_coord(0, i).unwrap();
            let tex_coords = vec2(v.x, v.y);
            
            let color = Vec4::ONE;

            vertices.push(Vertex { position: pos, color, tex_coords, normal });
        }

        let mut mesh = Mesh::new(
            vertices,
            indices,
            Shader::new("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs")
        );

        mesh.set_texture(texture);
        meshes.push(mesh);
    }

    for mesh in meshes.iter_mut() {
        for face in mesh.indices.chunks_mut(3) {
            face.reverse();
        }
    }

    meshes
}
