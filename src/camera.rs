use std::collections::HashMap;

use glam::{vec3, Mat4, Vec3};
use glfw::{Action, Key};

pub static mut PROJ_MATRIX: Mat4 = Mat4::IDENTITY;
pub static mut VIEW_MATRIX: Mat4 = Mat4::IDENTITY;

pub enum CameraMovement{
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub struct Camera{
    view: Mat4,

    pub position: Vec3,
    front: Vec3,
    right: Vec3,
    up: Vec3,

    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,

    pub first_mouse: bool,
}

impl Camera{
    pub fn new() -> Self{
        let position = vec3(0., 0., 3.);

        let (pitch, yaw): (f32, f32) = (0.0, -90.0);
        
        let target = vec3(0.0, 0.0, -1.0);
        let mut direction = (position - target).normalize();
        direction.x = yaw.to_radians().cos() * pitch.to_radians().cos();
        direction.y = pitch.to_radians().sin();
        direction.z = yaw.to_radians().sin() * pitch.to_radians().cos();
        
        let right = Vec3::Y.cross(direction).normalize();
        let up = direction.cross(right);
        let front = direction.normalize();

        Camera{
            view: Mat4::look_at_rh(position, position + front, up),

            position,
            up,
            front,
            right,

            yaw: -90.0,
            pitch: 0.0,
            movement_speed: 10.,
            mouse_sensitivity: 0.3,
            zoom: 90.0,

            first_mouse: true,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4{
        self.view
    }

    pub fn update_matrix(&mut self, w: f32, h: f32){
        unsafe {
            VIEW_MATRIX = Mat4::look_at_rh(self.position, self.position+self.front, self.up);
            PROJ_MATRIX = create_perspective_projection_matrix(w, h, self.zoom.to_radians());
            self.view = VIEW_MATRIX;
        }
    }

    pub fn movement(&mut self, keyboard: HashMap<Key, Action>, dt: f32){
        if keyboard[&Key::W] == Action::Press || keyboard[&Key::W] == Action::Repeat{
            self.position += self.movement_speed * dt * self.front; 
        }
        if keyboard[&Key::S] == Action::Press || keyboard[&Key::S] == Action::Repeat{
            self.position -= self.movement_speed * dt * self.front; 
        }
        if keyboard[&Key::Space] == Action::Press || keyboard[&Key::Space] == Action::Repeat{
            self.position.y += self.movement_speed * dt; // * self.up;
        }
        if keyboard[&Key::LeftControl] == Action::Press || keyboard[&Key::LeftControl] == Action::Repeat{
            self.position.y -= self.movement_speed * dt; // * self.up;
        }
        if keyboard[&Key::A] == Action::Press || keyboard[&Key::A] == Action::Repeat{
            self.position -= self.movement_speed * dt * self.front.cross(self.up).normalize(); 
        }
        if keyboard[&Key::D] == Action::Press || keyboard[&Key::D] == Action::Repeat{
            self.position += self.movement_speed * dt * self.front.cross(self.up).normalize(); 
        }
    }

    pub fn scroll_callback(&mut self, yoffset: f32){
        self.zoom -= yoffset*self.mouse_sensitivity;
        if self.zoom < 1.0{
            self.zoom = 1.0;
        }
        if self.zoom > 90.0{
            self.zoom = 90.0; 
        }
    }

    pub fn process_mouse_movement(&mut self, mut xoff: f32, mut yoff: f32, constrain_pitch: bool){
        if !self.first_mouse {
            xoff *= self.mouse_sensitivity;
            yoff *= self.mouse_sensitivity;

            self.yaw += xoff;
            self.pitch += yoff;

            // make sure that when pitch is out of bounds, screen doesn't get flipped
            if constrain_pitch
            {
                if self.pitch > 89.0{
                    self.pitch = 89.0;
                }
                if self.pitch < -89.0{
                    self.pitch = -89.0;
                }
            }
        }
        else {
            self.first_mouse = false;
        }

        // update Front, Right and Up Vectors using the updated Euler angles
        self.update_camera_vectors();
    }

    pub fn update_camera_vectors(&mut self) {
        // calculate the new Front vector
        let mut front = Vec3::ZERO;
        front.x = f32::cos(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));
        front.y = f32::sin(f32::to_radians(self.pitch));
        front.z = f32::sin(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));
        self.front = front.normalize();
    
        // Define the world up vector (assuming Y is up in your world)
        let world_up = Vec3::Y;
    
        // Recalculate the Right and Up vectors
        self.right = Vec3::normalize(Vec3::cross(self.front, world_up));
        self.up = Vec3::normalize(Vec3::cross(self.right, self.front));
    }    
}

pub fn create_perspective_projection_matrix(w: f32, h: f32, fov: f32) -> Mat4 {
    let aspect_ratio = w as f32 / h as f32;
    let fov_y = fov; // 45 degree field of view
    let near = 0.1; // Near clipping plane
    let far = 100.0; // Far clipping plane

    Mat4::perspective_rh_gl(fov_y, aspect_ratio, near, far)
}
