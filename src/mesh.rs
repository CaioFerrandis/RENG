use std::{ffi::CString, ptr};

use gl::{*, types::*};
use glam::{vec4, Mat4, Vec2, Vec3, Vec4};

use crate::{bind_buffer, camera::{PROJ_MATRIX, VIEW_MATRIX}, gen_attrib_pointers, light::LIGHTS, shader::Shader, texture::make_tex, transform::Transform, vertex::Vertex, window};

pub struct Mesh{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub shader: Shader,

    pub texture: u32,

    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
}

impl Mesh{
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, shader: Shader) -> Self{
        let vbo = 0;
        let vao = 0;
        let ebo = 0;

        let texture = 0;

        let m = 
        Mesh {
            vertices,
            indices,
            shader,

            texture,

            vao,
            vbo,
            ebo,
        };
        m
    }

    pub fn set_texture(&mut self, texture: u32){
        self.texture = texture;
    }

    pub fn setup_mesh(&mut self){
        unsafe{
            GenVertexArrays(1, &mut self.vao);
            GenBuffers(1, &mut self.vbo);

            BindVertexArray(self.vao);

            bind_buffer!(gl::ARRAY_BUFFER, self.vbo, self.vertices);
            gen_attrib_pointers!(Vertex, 0 => position:3, 1 => color:4, 2 => tex_coords:2, 3 => normal: 3);

            GenBuffers(1, &mut self.ebo);
            bind_buffer!(gl::ELEMENT_ARRAY_BUFFER, self.ebo, self.indices);
            BindVertexArray(0);
        }
    }

    pub fn update_mesh(&mut self) {
        // Bind the VAO and VBO
        unsafe {
            BindVertexArray(self.vao);
            bind_buffer!(gl::ARRAY_BUFFER, self.vbo, self.vertices);
    
            // Only update the vertices (using glBufferSubData for dynamic updates)
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                self.vertices.as_ptr() as *const _,
            );
    
            // If indices change, update them too
            bind_buffer!(gl::ELEMENT_ARRAY_BUFFER, self.ebo, self.indices);
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                (self.indices.len() * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _,
            );
    
            BindVertexArray(0);
        }
    }    

    pub fn translate(&mut self, change: Vec3){
        for i in self.vertices.iter_mut(){
            i.position += change;
        }
    }

    pub fn set_position(&mut self, position: Vec3){
        for i in self.vertices.iter_mut(){
            i.position += i.position - position;
        }
    }

    pub fn scale3D(&mut self, scale: Vec3){
        let pivot = self.vertices[0].position;

        for i in self.vertices.iter_mut() {
            let translated_position = i.position - pivot;
            
            let scaled_position = translated_position * scale;
    
            i.position = scaled_position + pivot;
        }
    }

    pub fn scale(&mut self, scale: f32){
        let pivot = self.vertices[0].position;

        for i in self.vertices.iter_mut() {
            let translated_position = i.position - pivot;
            
            let scaled_position = translated_position * scale;
    
            i.position = scaled_position + pivot;
        }
    }

    pub fn set_color(&mut self, color: Vec4){
        for vert in self.vertices.iter_mut(){
            vert.color = color;
        }
    }

    pub fn get_color(&self) -> Vec4{
        self.vertices[0].color
    }

    pub fn set_shader(&mut self, vertexPath: &str, fragmentPath: &str){
        self.shader = Shader::new(vertexPath, fragmentPath)
    }

    pub fn empty() -> Mesh{
        Mesh::new(vec![Vertex{position: Vec3::ZERO, color: Vec4::ONE, tex_coords: Vec2::ZERO, normal: Vec3::ZERO}],
            vec![0],
            Shader::new("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs"))
    }

    // TODO: implement this later!!!
    //pub fn set_shader(&mut self, )

    pub fn draw(&self, view_position: Vec3, transform: Transform){
        unsafe {
            self.shader.useProgram();
            BindVertexArray(self.vao);

            self.shader.uniform_mat4fv(&CString::new("projection").expect("error when sending projection matrix to shader"), &PROJ_MATRIX.to_cols_array());
            
            self.shader.uniform_mat4fv(&CString::new("view").expect("error when sending view matrix to shader"), &VIEW_MATRIX.to_cols_array());

            self.shader.setVector3(&CString::new("viewPos").expect("error when sending view position to shader"), &view_position);

            self.shader.uniform_mat4fv(&CString::new("model").expect("error when sending model matrix to shader"), &get_model_matrix(transform).to_cols_array());

            self.shader.setVector4(&CString::new("color").expect("error when sending mesh color to shader"), &self.vertices[0].color);
            
            let mut i = 0;
            for light in LIGHTS.iter(){
                self.shader.setVector3(&CString::new(format!("lightColor[{}]", i)).expect("error when sending light pos to shader"), &light.color);
                self.shader.setVector3(&CString::new(format!("lightPos[{}]", i)).expect("error when sending light pos to shader"), &light.position);
                i += 1;
            }

            BindTexture(gl::TEXTURE_2D, self.texture);

            DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            BindVertexArray(0);
            UseProgram(0);
        };
    }
}

pub fn get_model_matrix(transform: Transform) -> Mat4{
    let translation_matrix = Mat4::from_translation(transform.position);
    let rotation_matrix = Mat4::from_quat(transform.rotation);
    let scale_matrix = Mat4::from_scale(transform.scale);

    translation_matrix * rotation_matrix * scale_matrix
}
