mod vertex;
mod mesh;
mod shader;
mod macros;
mod window;
mod game_object;
mod transform;
mod shapes;
mod camera;
mod texture;
mod light;
mod line;

use core::f32;
use std::{collections::HashMap, thread, time::Duration};

use game_object::GameObject;
use glam::{vec2, vec3, vec4, Quat, Vec2, Vec3, Vec4};

use glfw::{Action, Key};
use light::{Light, LIGHTS};
use line::Line;
use mesh::Mesh;
use shapes::Shapes;
use texture::make_tex;
use window::Window;

// settings
const W: u32 = 800;
const H: u32 = 600;

pub fn main() {
    let mut window = Window::new(W, H);
    window.set_caption("ulala babe babe cmon");

    let mut texture_pack: HashMap<usize, u32> = HashMap::default();
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));
    texture_pack.insert(2, make_tex("src/textures/container.jpg"));
    texture_pack.insert(3, make_tex("src/textures/grass.png"));

    unsafe{
        LIGHTS.push(Light { position: vec3(0., 0., 0.), color: vec3(1., 1., 1.)/3. });
    }

    let mut test = GameObject::<Mesh>::new(Mesh::empty());
    test.set_shape(Shapes::Cube);
    test.set_texture(texture_pack[&2]);
    test.setup_mesh();

    let mut changed_cursor = false;
    
    window.lock_cursor();
    while !window.should_close() {
        let view_position = window.camera.position;
        window.clear_screen();

        window.camera.movement(window.keyboard.clone(), window.dt);

        unsafe{
            LIGHTS[0].position = window.camera.position;
        }

        if window.keyboard[&Key::LeftAlt] == Action::Press{
            if !changed_cursor{
                window.lock_cursor();
                changed_cursor = true;
            }
        }
        else{
            changed_cursor = false;
        }

        test.draw(&view_position);

        window.update();
    }
}
