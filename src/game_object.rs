use std::collections::HashMap;

use glam::{EulerRot, Mat4, Quat, Vec3, Vec4};

use crate::{line::Line, mesh::Mesh, shapes::{make_shape, Shapes}, texture::make_tex, transform::{self, Transform}, vertex::Vertex};

#[derive(Clone, Copy)]
pub struct GameObject<T>{
    pub object: T,
    pub transform: Transform,
    pub color: Vec4,
    pub shape: Shapes,
}

impl GameObject<Mesh>{
    pub fn new(mesh: Mesh) -> Self{
        let transform = Transform::new();

        GameObject{
            object: mesh,
            transform,
            color: Vec4::ONE,
            shape: Shapes::Empty,
        }
    }

    pub fn draw(&self, view_position: &Vec3){
        self.object.draw(*view_position, self.transform);
    }

    pub fn set_shape(&mut self, new_shape: Shapes){
        self.object = make_shape(new_shape, self.transform, self.color);
        self.object.update_mesh();
        self.shape = new_shape;
    }

    pub fn set_position(&mut self, position: Vec3){
        self.transform.position = position;
    }

    pub fn translate(&mut self, change: Vec3){
        self.transform.position += change;
    }

    pub fn scale(&mut self, scale: f32){
        self.transform.scale *= scale;
    }

    pub fn scale3D(&mut self, scale: Vec3){
        self.transform.scale = scale;
    }

    pub fn rotate(&mut self, rotation: Vec3){
        self.transform.rotation *= Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
    }

    pub fn set_rotation(&mut self, rotation: Vec3){
        self.transform.rotation = Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
    }

    pub fn set_color(&mut self, color: Vec4){
        let fixed_color = color.clamp(Vec4::ZERO, Vec4::ONE);

        self.color = fixed_color;
        self.object.set_color(fixed_color);
        self.object.update_mesh();
    }

    pub fn get_color(&self) -> Vec4{
        self.color
    }

    pub fn set_texture(&mut self, texture: u32){
        self.object.texture = texture;
    }

    pub fn set_shader(&mut self, vert_path: &str, frag_path: &str){
        self.object.set_shader(vert_path, frag_path);
    }

    pub fn setup_mesh(&mut self){
        self.object.setup_mesh();
    }
}

impl GameObject<Line>{
    pub fn new(begin: Vec3, end: Vec3, bidimensional: bool) -> Self{
        Self{
            object: Line::new(begin, end, Vec4::ONE, bidimensional),
            transform: Transform::new(),
            color: Vec4::ONE,
            shape: Shapes::Line,
        }
    }

    pub fn set_begin(&mut self, begin: Vec3){
        self.object.begin = begin;
        self.object.update();
    }

    pub fn set_end(&mut self, end: Vec3){
        self.object.end = end;
        self.object.update();
    }

    pub fn set_color(&mut self, color: Vec4){
        self.object.color = color;
        self.object.update();
    }

    pub fn setup_mesh(&mut self){
        self.object.mesh.setup_mesh();
    }

    pub fn draw(&self, view_position: Vec3){
        self.object.draw(view_position, self.transform);
    }
}
