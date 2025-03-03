use std::{cell::RefCell, rc::Weak, sync::Arc};

use glam::{vec3, EulerRot, Mat3, Mat4, Quat, Vec3, Vec4};

use crate::{line::Line, mesh::Mesh, shapes::{make_shape, Shapes}, transform::Transform};

#[derive(Clone)]
pub struct GameObject<T>{
    pub object: T,
    pub transform: Transform,
    pub color: Vec4,
    pub shape: Shapes,
    pub tag: String,
    pub name: String,
    pub parent: Option<Weak<RefCell<(GameObject<T>)>>>,
    pub children: Vec<Arc<RefCell<GameObject<T>>>>,
}

impl GameObject<Vec<Mesh>>{
    pub fn new(meshes: Vec<Mesh>) -> Self{
        let transform = Transform::new();

        GameObject{
            object: meshes,
            transform,
            color: Vec4::ONE,
            shape: Shapes::Empty,
            tag: "".to_owned(),
            name: "".to_owned(),
            parent: None,
            children: Vec::new()
        }
    }

    pub fn draw(&mut self, view_position: &Vec3){
        for mesh in self.object.iter(){
            mesh.draw(*view_position, self.transform);
        }
        for child in self.children.iter_mut(){
            child.borrow_mut().draw(&view_position);
        }
    }

    pub fn set_shape(&mut self, new_shape: Shapes){
        self.object = make_shape(new_shape, self.transform, self.color);
        self.object[0].update_mesh();
        self.shape = new_shape;
    }

    pub fn add_shapes(&mut self, meshes: Vec<Mesh>){
        for mesh in meshes.iter(){
            self.object.push(mesh.clone());
        }
    }

    pub fn set_position(&mut self, position: Vec3){
        let delta_pos = position - self.transform.position;
        self.transform.position = position;

        for child in self.children.iter_mut(){
            child.borrow_mut().translate(delta_pos);
        }
    }

    pub fn translate(&mut self, change: Vec3){
        self.transform.position += change;
        
        for child in self.children.iter_mut(){
            child.borrow_mut().translate(change);
        }
    }

    pub fn scale(&mut self, scale: f32){
        self.transform.scale *= scale;

        for child in self.children.iter_mut(){
            child.borrow_mut().scale(scale);
        }
    }

    pub fn scale3D(&mut self, scale: Vec3){
        self.transform.scale = scale;

        for child in self.children.iter_mut(){
            child.borrow_mut().scale3D(scale);
        }
    }

    pub fn local_rotate(&mut self, rotation: Vec3){
        self.transform.rotation *= Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);

        for child in self.children.iter_mut(){
            
            let translate_to_origin = Mat4::from_translation(-self.transform.position);
            let quat = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
            let rotation_matrix = Mat4::from_quat(quat);
            let translate_back = Mat4::from_translation(self.transform.position);
            
            let mut c = child.borrow_mut();
            let transform = translate_back * rotation_matrix * translate_to_origin;

            let new_pos = transform.transform_point3(c.transform.position);
            c.local_rotate(rotation);
            c.set_position(new_pos);
        }
    }

    pub fn global_rotate(&mut self, rotation: Vec3){
        let quat_rotation = Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);

        self.transform.rotation = quat_rotation * self.transform.rotation;
        self.transform.position = quat_rotation * self.transform.position;

        for child in self.children.iter_mut(){
            
            let translate_to_origin = Mat4::from_translation(-self.transform.position);
            let quat = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
            let rotation_matrix = Mat4::from_quat(quat);
            let translate_back = Mat4::from_translation(self.transform.position);
            
            let mut c = child.borrow_mut();
            let transform = translate_back * rotation_matrix * translate_to_origin;

            let new_pos = transform.transform_point3(c.transform.position);
            c.global_rotate(rotation);
            c.set_position(new_pos);
        }
    }

    pub fn set_rotation(&mut self, rotation: Vec3){
        let new_rotation = Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        let delta_rotation = new_rotation * self.transform.rotation.conjugate();

        self.transform.rotation = new_rotation;

        for child in self.children.iter_mut(){
            let mut c = child.borrow_mut();
            
            c.set_rotation(rotation);
            c.transform.position = self.transform.position + delta_rotation * (c.transform.position - self.transform.position);
        }
    }

    pub fn look_at_raw(&mut self, target_position: Vec3) -> Quat {
        let forward = (target_position - self.transform.position).normalize();

        let right = vec3(0., 1., 0.).cross(forward).normalize();

        let new_up = forward.cross(right);

        let rotation_matrix = Mat3::from_cols(right, new_up, forward);

        for child in self.children.iter_mut(){
            child.borrow_mut().set_rotation(Vec3::from(rotation_matrix.to_euler(EulerRot::XYZ)));
        }

        Quat::from_mat3(&rotation_matrix)
    }

    pub fn look_at(&mut self, target_position: Vec3) {
        let forward = (target_position - self.transform.position).normalize();

        let right = vec3(0., 1., 0.).cross(forward).normalize();

        let new_up = forward.cross(right);

        let rotation_matrix = Mat3::from_cols(right, new_up, forward);
        
        self.set_rotation(Vec3::from(Quat::from_mat3(&rotation_matrix).to_euler(EulerRot::XYZ)));
    }

    pub fn set_color(&mut self, color: Vec4){
        let fixed_color = color.clamp(Vec4::ZERO, Vec4::ONE);

        for mesh in self.object.iter_mut(){
            self.color = fixed_color;
            mesh.set_color(fixed_color);
            mesh.update_mesh();
        }
    }

    pub fn get_color(&self) -> Vec4{
        self.color
    }

    pub fn set_texture(&mut self, texture: u32){
        for mesh in self.object.iter_mut(){
            mesh.set_texture(texture);
        }
    }

    pub fn set_shader(&mut self, vert_path: &str, frag_path: &str){
        for mesh in self.object.iter_mut(){
            mesh.set_shader(vert_path, frag_path);
        }
    }

    pub fn setup_meshes(&mut self){
        for mesh in self.object.iter_mut(){
            mesh.setup_mesh();
        }
    }
}

impl GameObject<Line>{
    pub fn new(begin: Vec3, end: Vec3, bidimensional: bool) -> Self{
        Self{
            object: Line::new(begin, end, Vec4::ONE, bidimensional),
            transform: Transform::new(),
            color: Vec4::ONE,
            shape: Shapes::Line,
            tag: "".to_owned(),
            name: "".to_owned(),
            parent: None,
            children: Vec::new(),
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

    pub fn draw(&mut self, view_position: &Vec3){
        self.object.draw(*view_position, self.transform);

        for child in self.children.iter_mut(){
            child.borrow_mut().draw(&view_position);
        }
    }
}

pub fn quickGO(shape: Shapes, texture: u32) -> GameObject<Vec<Mesh>>{
    let mut go = GameObject::<Vec<Mesh>>::new(Mesh::empty());

    match shape{
        Shapes::Cube => go.set_shape(Shapes::Cube),
        Shapes::Sphere => go.set_shape(Shapes::Sphere),
        Shapes::Empty => go.set_shape(Shapes::Empty),
        _ => go.set_shape(Shapes::Cube),
    }
    go.set_texture(texture);
    go.setup_meshes();

    go
}
