use glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

use crate::{mesh::Mesh, shader::Shader, transform::Transform, vertex::Vertex};

pub enum Shapes{
    Circle,
    Quad,
    Cube,
    Sphere,
    Triangle,
    Line,
}

pub fn make_shape(shape: Shapes, transform: Transform, color: Vec4) -> Mesh{
    match shape{
        Shapes::Circle => {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();

            // The center of the circle
            vertices.push(Vertex {
                position: transform.position,
                color,
                tex_coords: vec2(0.5, 0.5),
                normal: vec3(0.0, 0.0, 1.0), // Normal pointing up
            });

            let segments = 64;
            let radius = 1.0;

            // Generate vertices for the circle
            for i in 0..segments {
                let angle = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
                
                let sin = angle.sin();
                let cos = angle.cos();
                
                let x = (radius * transform.scale.x * cos) + transform.position.x;
                let y = (radius * transform.scale.y * sin) + transform.position.y;

                let uv_x = (cos * 0.5 + 0.5) as f32;
                let uv_y = (sin * 0.5 + 0.5) as f32;

                vertices.push(Vertex { 
                    position: vec3(x, y, 0.0), 
                    color, 
                    tex_coords: vec2(uv_x, uv_y), 
                    normal: vec3(0.0, 0.0, 1.0), // Normal pointing up
                });
            }

            // Generate indices for the circle
            for i in 1..segments {
                indices.push(0);
                indices.push(i as u32);
                indices.push((i + 1) as u32 % segments as u32);
            }

            indices.push(0);
            indices.push((segments - 1) as u32);
            indices.push(segments);

            indices.push(0);
            indices.push(segments as u32);
            indices.push(1);

            Mesh::new(vertices, indices, Shader::new("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs"))
        }

        Shapes::Sphere => {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
        
            let segments = 64; // Number of horizontal divisions (longitude)
            let rings = 64;    // Number of vertical divisions (latitude)
            let radius = 1.0;
        
            // Generate vertices for the sphere
            for i in 0..=rings {
                let theta = i as f32 * std::f32::consts::PI / rings as f32; // from 0 to PI (latitude)
                let sin_theta = theta.sin();
                let cos_theta = theta.cos();
        
                for j in 0..=segments {
                    let phi = j as f32 * 2.0 * std::f32::consts::PI / segments as f32; // from 0 to 2PI (longitude)
                    let sin_phi = phi.sin();
                    let cos_phi = phi.cos();
        
                    let x = radius * sin_theta * cos_phi * transform.scale.x + transform.position.x;
                    let y = radius * sin_theta * sin_phi * transform.scale.y + transform.position.y;
                    let z = radius * cos_theta * transform.scale.z + transform.position.z;
        
                    // Normal is the direction from the center of the sphere
                    let normal = vec3(x - transform.position.x, y - transform.position.y, z - transform.position.z).normalize();
        
                    let u = j as f32 / segments as f32;
                    let v = i as f32 / rings as f32;
        
                    vertices.push(Vertex {
                        position: vec3(x, y, z),
                        color,
                        tex_coords: vec2(u, v),
                        normal,
                    });
                }
            }
        
            // Generate indices for the sphere
            for i in 0..rings {
                for j in 0..segments {
                    let first = i * (segments + 1) + j;
                    let second = first + segments + 1;
        
                    indices.push(first as u32);
                    indices.push(second as u32);
                    indices.push((first + 1) as u32);
        
                    indices.push(second as u32);
                    indices.push((second + 1) as u32);
                    indices.push((first + 1) as u32);
                }
            }
        
            Mesh::new(vertices, indices, Shader::new("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs"))
        }        

        Shapes::Quad => {
            let half_width = transform.scale.x / 2.0;
            let half_height = transform.scale.y / 2.0;

            let vertices = vec![
                // Bottom-left
                Vertex {
                    position: vec3(-half_width + transform.position.x, -half_height + transform.position.y, 0.0),
                    color,
                    tex_coords: vec2(0.0, 0.0),
                    normal: vec3(0.0, 0.0, 1.0),  // Normal pointing up
                },
                // Bottom-right
                Vertex {
                    position: vec3(half_width + transform.position.x, -half_height + transform.position.y, 0.0),
                    color,
                    tex_coords: vec2(1.0, 0.0),
                    normal: vec3(0.0, 0.0, 1.0),  // Normal pointing up
                },
                // Top-right
                Vertex {
                    position: vec3(half_width + transform.position.x, half_height + transform.position.y, 0.0),
                    color,
                    tex_coords: vec2(1.0, 1.0),
                    normal: vec3(0.0, 0.0, 1.0),  // Normal pointing up
                },
                // Top-left
                Vertex {
                    position: vec3(-half_width + transform.position.x, half_height + transform.position.y, 0.0),
                    color,
                    tex_coords: vec2(0.0, 1.0),
                    normal: vec3(0.0, 0.0, 1.0),  // Normal pointing up
                },
            ];

            let indices = vec![
                0, 1, 2,  // First triangle
                2, 3, 0,  // Second triangle
            ];

            Mesh::new(vertices, indices, Shader::new("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs"))
        }

        Shapes::Cube => {
            let half_width = transform.scale.x / 2.0;
            let half_height = transform.scale.y / 2.0;
            let half_depth = transform.scale.z / 2.0;

            let vertices = vec![
                // Front face (normal: +Z)
                Vertex { position: vec3(-half_width + transform.position.x, -half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 0.0), normal: vec3(0.0, 0.0, 1.0) },
                Vertex { position: vec3(half_width + transform.position.x, -half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 0.0), normal: vec3(0.0, 0.0, 1.0) },
                Vertex { position: vec3(half_width + transform.position.x, half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 1.0), normal: vec3(0.0, 0.0, 1.0) },
                Vertex { position: vec3(-half_width + transform.position.x, half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 1.0), normal: vec3(0.0, 0.0, 1.0) },

                // Back face (normal: -Z)
                Vertex { position: vec3(-half_width + transform.position.x, -half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 0.0), normal: vec3(0.0, 0.0, -1.0) },
                Vertex { position: vec3(half_width + transform.position.x, -half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 0.0), normal: vec3(0.0, 0.0, -1.0) },
                Vertex { position: vec3(half_width + transform.position.x, half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 1.0), normal: vec3(0.0, 0.0, -1.0) },
                Vertex { position: vec3(-half_width + transform.position.x, half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 1.0), normal: vec3(0.0, 0.0, -1.0) },

                // Left face (normal: -X)
                Vertex { position: vec3(-half_width + transform.position.x, -half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 0.0), normal: vec3(-1.0, 0.0, 0.0) },
                Vertex { position: vec3(-half_width + transform.position.x, -half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 0.0), normal: vec3(-1.0, 0.0, 0.0) },
                Vertex { position: vec3(-half_width + transform.position.x, half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 1.0), normal: vec3(-1.0, 0.0, 0.0) },
                Vertex { position: vec3(-half_width + transform.position.x, half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 1.0), normal: vec3(-1.0, 0.0, 0.0) },

                // Right face (normal: +X)
                Vertex { position: vec3(half_width + transform.position.x, -half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 0.0), normal: vec3(1.0, 0.0, 0.0) },
                Vertex { position: vec3(half_width + transform.position.x, -half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 0.0), normal: vec3(1.0, 0.0, 0.0) },
                Vertex { position: vec3(half_width + transform.position.x, half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 1.0), normal: vec3(1.0, 0.0, 0.0) },
                Vertex { position: vec3(half_width + transform.position.x, half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 1.0), normal: vec3(1.0, 0.0, 0.0) },

                // Top face (normal: +Y)
                Vertex { position: vec3(-half_width + transform.position.x, half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 0.0), normal: vec3(0.0, 1.0, 0.0) },
                Vertex { position: vec3(half_width + transform.position.x, half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 0.0), normal: vec3(0.0, 1.0, 0.0) },
                Vertex { position: vec3(half_width + transform.position.x, half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 1.0), normal: vec3(0.0, 1.0, 0.0) },
                Vertex { position: vec3(-half_width + transform.position.x, half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 1.0), normal: vec3(0.0, 1.0, 0.0) },

                // Bottom face (normal: -Y)
                Vertex { position: vec3(-half_width + transform.position.x, -half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 0.0), normal: vec3(0.0, -1.0, 0.0) },
                Vertex { position: vec3(half_width + transform.position.x, -half_height + transform.position.y, -half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 0.0), normal: vec3(0.0, -1.0, 0.0) },
                Vertex { position: vec3(half_width + transform.position.x, -half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(1.0, 1.0), normal: vec3(0.0, -1.0, 0.0) },
                Vertex { position: vec3(-half_width + transform.position.x, -half_height + transform.position.y, half_depth + transform.position.z), color, tex_coords: Vec2::new(0.0, 1.0), normal: vec3(0.0, -1.0, 0.0) },
            ];

            let indices = vec![
                // Front face
                0, 1, 2, 2, 3, 0,
                // Back face
                4, 5, 6, 6, 7, 4,
                // Left face
                8, 9, 10, 10, 11, 8,
                // Right face
                12, 13, 14, 14, 15, 12,
                // Top face
                16, 17, 18, 18, 19, 16,
                // Bottom face
                20, 21, 22, 22, 23, 20,
            ];

            Mesh::new(vertices, indices, Shader::new("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs"))
        }

        Shapes::Triangle => {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
        
            // Define the three points of the triangle
            let p1 = vec3(transform.position.x, transform.position.y + transform.scale.y, 0.0);  // Top vertex
            let p2 = vec3(transform.position.x - transform.scale.x, transform.position.y - transform.scale.y, 0.0); // Bottom left vertex
            let p3 = vec3(transform.position.x + transform.scale.x, transform.position.y - transform.scale.y, 0.0); // Bottom right vertex
        
            // Define the vertices with positions, color, UV coordinates, and normal
            vertices.push(Vertex {
                position: p1,
                color,
                tex_coords: vec2(0.5, 1.0), // UV for the top
                normal: vec3(0.0, 0.0, 1.0), // Normal pointing up (2D)
            });
        
            vertices.push(Vertex {
                position: p2,
                color,
                tex_coords: vec2(0.0, 0.0), // UV for bottom left
                normal: vec3(0.0, 0.0, 1.0),
            });
        
            vertices.push(Vertex {
                position: p3,
                color,
                tex_coords: vec2(1.0, 0.0), // UV for bottom right
                normal: vec3(0.0, 0.0, 1.0),
            });
        
            // Indices for the triangle (just one triangle)
            indices.push(0);
            indices.push(1);
            indices.push(2);
        
            Mesh::new(vertices, indices, Shader::new("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs"))
        }

        _ => {
            Mesh::empty()
        }
    }
}
