use std::ffi::CString;

use gl::{BindTexture, BindVertexArray, UseProgram};
use glam::Vec3;

use crate::{camera::{PROJ_MATRIX, VIEW_MATRIX}, get_model_matrix, GameObject, Mesh, Transform, LIGHTS};

pub fn create_instance_buffer(instance_data: &[Transform]) -> u32 {
    let mut instance_buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut instance_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, instance_buffer);
        

        let instance_matrices: Vec<glam::Mat4> = instance_data
            .iter()
            .map(|t| get_model_matrix(*t))
            .collect();

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (instance_matrices.len() * std::mem::size_of::<glam::Mat4>()) as gl::types::GLsizeiptr,
            instance_matrices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        let attrib_location = 4;
        for i in 0..4 {
            gl::EnableVertexAttribArray(attrib_location + i);
            gl::VertexAttribPointer(
                attrib_location + i,
                4,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<glam::Mat4>() as i32,
                (i as usize * std::mem::size_of::<glam::Vec4>()) as *const gl::types::GLvoid,
            );
            gl::VertexAttribDivisor(attrib_location + i, 1);
        }
    }

    instance_buffer
}

impl GameObject<Vec<Mesh>> {
    pub fn bind_instanced(&self, view_position: Vec3) {
        unsafe {
            for mesh in self.object.iter() {
                mesh.shader.useProgram();
                BindVertexArray(mesh.vao);

                // Passando as matrizes de projeção e visão para o shader
                mesh.shader.uniform_mat4fv(&CString::new("projection").expect("error when sending projection matrix to shader"), &PROJ_MATRIX.to_cols_array());
                mesh.shader.uniform_mat4fv(&CString::new("view").expect("error when sending view matrix to shader"), &VIEW_MATRIX.to_cols_array());
                mesh.shader.setVector3(&CString::new("viewPos").expect("error when sending view position to shader"), &view_position);

                // Enviando as informações das luzes
                let mut i = 0;
                for light in LIGHTS.iter() {
                    mesh.shader.setVector3(&CString::new(format!("lightColor[{}]", i)).expect("error when sending light color to shader"), &light.color);
                    mesh.shader.setVector3(&CString::new(format!("lightPos[{}]", i)).expect("error when sending light position to shader"), &light.position);
                    i += 1;
                }
                
                // Vinculando a textura para o mesh
                BindTexture(gl::TEXTURE_2D, mesh.texture);
            }
        }
    }
    
    pub fn draw_instanced(instance_buffer: u32, instance_count: i32, indices_count: usize) {
        unsafe {
            // Aqui, chamamos uma única vez para todas as instâncias de todos os meshes
            gl::BindBuffer(gl::ARRAY_BUFFER, instance_buffer); // Associando o buffer de instâncias

            // Chamando o desenho das instâncias
            gl::DrawElementsInstanced(
                gl::TRIANGLES,                    // Tipo de primitiva
                indices_count as i32, // Número de índices (usa o do primeiro mesh como exemplo)
                gl::UNSIGNED_INT,                  // Tipo de índice
                std::ptr::null(),                  // Deslocamento dos índices
                instance_count,                    // Número de instâncias
            );

            BindVertexArray(0);
            UseProgram(0);
        }
    }
}
