use std:: collections::HashMap;
use std::vec;

use reng::game_object::GameObject;
use glam::{vec3, vec4, Vec3};

use glfw::Key;
use reng::light::{Light, LIGHTS};
use reng::{ecs, quick_go};
use reng::shapes::Shapes;
use reng::texture::make_tex;
use reng::window::Window;
use std::cell::RefCell;

// settings
const W: u32 = 800;
const H: u32 = 600;

pub fn main() {
    let mut window = Window::new(W, H);

    let mut texture_pack: HashMap<usize, u32> = HashMap::default();
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));
    texture_pack.insert(2, make_tex("src/textures/container.jpg"));

    let sphere = ecs!(quick_go(Shapes::Sphere, texture_pack[&1]));
    sphere.borrow_mut().set_color(vec4(1., 0., 0., 1.));

    let child = ecs!(quick_go(Shapes::Cube, texture_pack[&1]));
    child.borrow_mut().set_color(vec4(0., 1., 0., 1.));
    child.borrow_mut().translate(vec3(5., 0., 0.));
    sphere.borrow_mut().children.push(child);

    unsafe{
        LIGHTS.push(Light { position: vec3(0., 0., 0.), color: vec3(1., 1., 1.) });
    }

    let mut changed_cursor = false;

    window.lock_cursor();
    while !window.should_close() {
        let view_position = window.camera.position;
        
        unsafe{
            LIGHTS[0].position = view_position;
        }

        window.camera.movement(&window.keyboard, window.dt);

        if window.is_pressing(Key::LeftAlt) {
            if !changed_cursor{
                window.lock_cursor();
                changed_cursor = true;
            }
        }
        else {
            changed_cursor = false;
        }

        let vel = 10.;
        if window.is_pressing(Key::Up){
            sphere.borrow_mut().translate(vec3(0., window.dt*vel, 0.));
        }
        if window.is_pressing(Key::Down){
            sphere.borrow_mut().translate(vec3(0., -window.dt*vel, 0.));
        }
        if window.is_pressing(Key::Right){
            sphere.borrow_mut().translate(vec3(window.dt*vel, 0., 0.));
        }
        if window.is_pressing(Key::Left){
            sphere.borrow_mut().translate(vec3(-window.dt*vel, 0., 0.));
        }
        if window.is_pressing(Key::Q){
            sphere.borrow_mut().local_rotate(vec3(0., window.dt*vel/2., 0.));
        }
        if window.is_pressing(Key::E){
            sphere.borrow_mut().local_rotate(vec3(0., 0., window.dt*vel/2.));
        }
        
        window.clear_screen();
        
        sphere.borrow().draw(view_position);

        window.update();
    }
}
