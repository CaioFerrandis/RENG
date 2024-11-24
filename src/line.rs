use glam::{vec2, vec3, Vec3, Vec4};

use crate::{mesh::Mesh, shader::Shader, transform::Transform, vertex::Vertex};

pub struct Line{
    pub begin: Vec3,
    pub end: Vec3,
    pub color: Vec4,
    pub bidimensional: bool,
    pub mesh: Mesh,
}

impl Line{
    pub fn new(begin: Vec3, end: Vec3, color: Vec4, bidimensional: bool) -> Self{
        let mesh: Mesh;
        if bidimensional{
            mesh = make_line_2d(begin, end, color);
        }
        else{
            mesh = make_line_3d(begin, end, color);
        }
        
        Self{
            begin,
            end,
            color,
            bidimensional,
            mesh,
        }
    }

    pub fn update(&mut self){
        if self.bidimensional{
            update_line_2d(&mut self.mesh, self.begin, self.end, self.color);
        }
        else{
            update_line_3d(&mut self.mesh, self.begin, self.end, self.color);
        }
        self.mesh.update_mesh();
    }

    pub fn draw(&self, view_position: Vec3, transform: Transform){
        self.mesh.draw(view_position, transform);
    }
}

pub fn make_line_2d(begin: Vec3, end: Vec3, color: Vec4) -> Mesh {
    let mut vertices = Vec::new();
    let indices = vec![0, 1, 2, 1, 2, 3];

    let line_direction = (end - begin).normalize(); // Calculate direction of the line
    let width = 0.1; // Set the thickness of the line

    // Calculate the perpendicular vector to the line direction (in 2D, rotate by 90 degrees)
    let perpendicular = vec3(-line_direction.y, line_direction.x, 0.0) * width;

    // Define vertices with offsets based on the perpendicular vector
    vertices.push(Vertex {
        position: begin - perpendicular, // Bottom-left corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    vertices.push(Vertex {
        position: end - perpendicular, // Bottom-right corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    vertices.push(Vertex {
        position: begin + perpendicular, // Top-left corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    vertices.push(Vertex {
        position: end + perpendicular, // Top-right corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    Mesh::new(vertices, indices, Shader::new("src/shaders/basic_shader.vs", "src/shaders/basic_shader.fs"))
}

pub fn make_line_3d(begin: Vec3, end: Vec3, color: Vec4) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let segments = 16; // Number of segments for the cylinder (higher = smoother)
    let radius = 0.05; // Thickness of the cylinder

    let line_direction = (end - begin).normalize(); // Line direction

    // Arbitrary "up" vector that is not parallel to the line direction
    let up = if line_direction.dot(vec3(0.0, 1.0, 0.0)).abs() > 0.99 {
        vec3(1.0, 0.0, 0.0) // Use a different up vector if the line is vertical
    } else {
        vec3(0.0, 1.0, 0.0)
    };

    // Calculate a vector perpendicular to the line direction using cross product
    let perpendicular = line_direction.cross(up).normalize() * radius;
    let perpendicular2 = line_direction.cross(perpendicular).normalize() * radius;

    // Create the circular rings around `begin` and `end`
    for i in 0..segments {
        let angle = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let rotation = perpendicular * angle.cos() + perpendicular2 * angle.sin();

        // First ring of vertices (at `begin`)
        vertices.push(Vertex {
            position: begin + rotation,
            color,
            tex_coords: vec2(i as f32 / segments as f32, 0.0),
            normal: rotation.normalize(),
        });

        // Second ring of vertices (at `end`)
        vertices.push(Vertex {
            position: end + rotation,
            color,
            tex_coords: vec2(i as f32 / segments as f32, 1.0),
            normal: rotation.normalize(),
        });
    }

    // Generate indices to create a cylindrical surface by connecting the rings
    for i in 0..segments {
        let next = (i + 1) % segments;

        // First triangle of the quad
        indices.push(i as u32 * 2);         // Vertex from the first ring
        indices.push(next as u32 * 2);      // Next vertex from the first ring
        indices.push(i as u32 * 2 + 1);     // Corresponding vertex from the second ring

        // Second triangle of the quad
        indices.push(next as u32 * 2);      // Next vertex from the first ring
        indices.push(next as u32 * 2 + 1);  // Next vertex from the second ring
        indices.push(i as u32 * 2 + 1);     // Corresponding vertex from the second ring
    }

    Mesh::new(vertices, indices, Shader::new("src/shaders/basic_shader.vs", "src/shaders/basic_shader.fs"))
}

pub fn update_line_2d(mesh: &mut Mesh, begin: Vec3, end: Vec3, color: Vec4) {
    let mut vertices = Vec::new();
    let indices = vec![0, 1, 2, 1, 2, 3];

    let line_direction = (end - begin).normalize(); // Calculate direction of the line
    let width = 0.1; // Set the thickness of the line

    // Calculate the perpendicular vector to the line direction (in 2D, rotate by 90 degrees)
    let perpendicular = vec3(-line_direction.y, line_direction.x, 0.0) * width;

    // Define vertices with offsets based on the perpendicular vector
    vertices.push(Vertex {
        position: begin - perpendicular, // Bottom-left corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    vertices.push(Vertex {
        position: end - perpendicular, // Bottom-right corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    vertices.push(Vertex {
        position: begin + perpendicular, // Top-left corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    vertices.push(Vertex {
        position: end + perpendicular, // Top-right corner of the quad
        color,
        tex_coords: vec2(0.0, 0.0),
        normal: Vec3::ONE,
    });

    mesh.vertices = vertices;
    mesh.indices = indices;
    for vert in mesh.vertices.iter_mut(){
        vert.color = color;
    }
}

pub fn update_line_3d(mesh: &mut Mesh, begin: Vec3, end: Vec3, color: Vec4) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let segments = 16; // Number of segments for the cylinder (higher = smoother)
    let radius = 0.05; // Thickness of the cylinder

    let line_direction = (end - begin).normalize(); // Line direction

    // Arbitrary "up" vector that is not parallel to the line direction
    let up = if line_direction.dot(vec3(0.0, 1.0, 0.0)).abs() > 0.99 {
        vec3(1.0, 0.0, 0.0) // Use a different up vector if the line is vertical
    } else {
        vec3(0.0, 1.0, 0.0)
    };

    // Calculate a vector perpendicular to the line direction using cross product
    let perpendicular = line_direction.cross(up).normalize() * radius;
    let perpendicular2 = line_direction.cross(perpendicular).normalize() * radius;

    // Create the circular rings around `begin` and `end`
    for i in 0..segments {
        let angle = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let rotation = perpendicular * angle.cos() + perpendicular2 * angle.sin();

        // First ring of vertices (at `begin`)
        vertices.push(Vertex {
            position: begin + rotation,
            color,
            tex_coords: vec2(i as f32 / segments as f32, 0.0),
            normal: rotation.normalize(),
        });

        // Second ring of vertices (at `end`)
        vertices.push(Vertex {
            position: end + rotation,
            color,
            tex_coords: vec2(i as f32 / segments as f32, 1.0),
            normal: rotation.normalize(),
        });
    }

    // Generate indices to create a cylindrical surface by connecting the rings
    for i in 0..segments {
        let next = (i + 1) % segments;

        // First triangle of the quad
        indices.push(i as u32 * 2);         // Vertex from the first ring
        indices.push(next as u32 * 2);      // Next vertex from the first ring
        indices.push(i as u32 * 2 + 1);     // Corresponding vertex from the second ring

        // Second triangle of the quad
        indices.push(next as u32 * 2);      // Next vertex from the first ring
        indices.push(next as u32 * 2 + 1);  // Next vertex from the second ring
        indices.push(i as u32 * 2 + 1);     // Corresponding vertex from the second ring
    }

    mesh.vertices = vertices;
    mesh.indices = indices;
    for vert in mesh.vertices.iter_mut(){
        vert.color = color;
    }
}
