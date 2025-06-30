use core::f32;
use std::{cell::RefCell, collections::HashMap, rc::Rc, thread, time::Duration};

use rand::Rng;
use reng::game_object::GameObject;
use glam::{vec2, vec3, vec4, Quat, Vec2, Vec3, Vec4};

use glfw::{Action, Key};
use reng::light::{Light, LIGHTS};
use reng::line::Line;
use reng::mesh::Mesh;
use reng::{load_model, quick_go, Transform};
use reng::shapes::Shapes;
use reng::texture::make_tex;
use reng::window::Window;
use reng::shader::Shader;
use reng::instance_rendering::create_instance_buffer;

// settings
const W: u32 = 800;
const H: u32 = 600;

pub fn main() {
    let mut window = Window::new(W, H);

    let mut texture_pack: HashMap<usize, u32> = HashMap::default();
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));
    texture_pack.insert(2, make_tex("src/textures/container.jpg"));

    let mut sphere = quick_go(Shapes::Sphere, texture_pack[&0]);
    sphere.set_color(vec4(1., 0., 0., 1.));

    unsafe{
        LIGHTS.push(Light { position: vec3(0., 0., 0.), color: vec3(1., 1., 1.)/3. });
    }

    let mut changed_cursor = false;

    window.lock_cursor();
    while !window.should_close() {
        let view_position = window.camera.position;
        
        unsafe{
            LIGHTS[0].position = view_position;
        }

        window.movement();

        if window.keyboard[&Key::LeftAlt] == Action::Press {
            if !changed_cursor{
                window.lock_cursor();
                changed_cursor = true;
            }
        }
        else if window.keyboard[&Key::LeftAlt] == Action::Release {
            changed_cursor = false;
        }

        window.clear_screen();

        sphere.draw(view_position);

        window.update();
    }
}
