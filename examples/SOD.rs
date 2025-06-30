use std:: collections::HashMap;

use reng::game_object::GameObject;
use glam::{vec3, vec4, Vec3};

use glfw::Key;
use reng::light::{Light, LIGHTS};
use reng::line::Line;
use reng::second_order_dynamics::SecondOrderDynamics;
use reng::spring::Spring;
use reng::quick_go;
use reng::shapes::Shapes;
use reng::texture::make_tex;
use reng::window::Window;

// settings
const W: u32 = 800;
const H: u32 = 600;
const G: Vec3 = vec3(0., -9.81, 0.);

pub fn main() {
    let mut window = Window::new(W, H);

    let mut texture_pack: HashMap<usize, u32> = HashMap::default();
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));
    texture_pack.insert(2, make_tex("src/textures/container.jpg"));

    let mut sphere = quick_go(Shapes::Sphere, texture_pack[&1]);
    sphere.set_color(vec4(1., 0., 0., 1.));

    let mut quad = quick_go(Shapes::Cube, texture_pack[&1]);
    quad.set_color(vec4(0., 1., 0., 1.));
    quad.translate(vec3(5., 0., 0.));

    let mut target = Vec3::ZERO;

    let mut sod = SecondOrderDynamics::new(1., 0.1, 0., sphere.transform.position);
    let mut sod2 = SecondOrderDynamics::new(1., 0.1, 0., quad.transform.position);
    
    let mut spring = Spring::new(target, sphere.transform.position, 10., 1.);
    let mut spring2 = Spring::new(sphere.transform.position, quad.transform.position, 10., 1.);

    let mut line = GameObject::<Line>::new(target, sphere.transform.position, false);
    line.set_texture(texture_pack[&1]);
    line.setup_mesh();

    let mut line2 = GameObject::<Line>::new(sphere.transform.position, quad.transform.position, false);
    line2.set_texture(texture_pack[&1]);
    line2.setup_mesh();

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

        let vel = 15.;
        if window.is_pressing(Key::Up){
            target.y += window.dt*vel;
        }
        if window.is_pressing(Key::Down){
            target.y -= window.dt*vel;
        }
        if window.is_pressing(Key::Right){
            target.x += window.dt*vel;
        }
        if window.is_pressing(Key::Left){
            target.x -= window.dt*vel;
        }

        spring.p1 = target;
        spring.p2 = sphere.transform.position;
        spring.update();
        
        spring2.p1 = sphere.transform.position;
        spring2.p2 = quad.transform.position;
        spring2.update();
        
        sod.force += G;
        sod2.force += G;
        
        sphere.set_position(sod.update(window.dt, spring.p2, Option::None));

        quad.set_position(sod2.update(window.dt, spring2.p2, Option::None));

        sphere.set_position(sod.update(window.dt, spring2.p1, Option::None));

        line.set_begin(target);
        line.set_end(sphere.transform.position);

        line2.set_begin(sphere.transform.position);
        line2.set_end(quad.transform.position);
        
        window.update();

        window.clear_screen();

        line.draw(view_position);
        line2.draw(view_position);
        sphere.draw(view_position);
        quad.draw(view_position);
    }
}
