use core::f32;
use std::collections::HashMap;
use collision::Volume;
use glam::{vec3, vec4, Vec3};
use glfw::{Key, Action};
use reng::*;
use rapier3d::prelude::*;

fn main() {
    let mut window = Window::new(800, 600);
    window.set_caption("Drag'n Drop!'");
    
    let mut texture_pack: HashMap<usize, u32> = HashMap::default();
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));
    texture_pack.insert(2, make_tex("src/textures/grass.jpg"));
    texture_pack.insert(3, make_tex("src/textures/container.jpg"));

    unsafe{
        LIGHTS.push(Light { position: vec3(0., 0., 0.), color: vec3(1., 1., 1.)/3. });
    }

    let mut test_obj = GameObject::<Mesh>::new(Mesh::empty());
    test_obj.set_shape(Shapes::Cube);
    test_obj.set_texture(texture_pack[&3]);
    test_obj.setup_mesh();

    let mut hit = GameObject::<Mesh>::new(Mesh::empty());
    hit.set_shape(Shapes::Sphere);
    hit.set_texture(texture_pack[&3]);
    hit.set_color(vec4(1., 0., 0., 1.));
    hit.scale(0.1);
    hit.setup_mesh();

    let mut view_position = window.camera.position;

    let mut changed_cursor = false;

    let mut dragging = false;
    let mut ray = raycast(view_position, window.camera.get_forward_vec(), 100., vec![test_obj.clone()]);
    let mut dist = 0.;
    let mut last_click = 0.;
    let wait_between_clicks = 1.;

    while !window.should_close() {
        unsafe{
            LIGHTS[0].position = window.camera.position;
        }

        view_position = window.camera.position;

        window.clear_screen();

        window.camera.movement(window.keyboard.clone(), window.dt);

        if window.keyboard[&Key::LeftAlt] == Action::Press{
            if !changed_cursor{
                window.lock_cursor();
                changed_cursor = true;
            }
        }
        else{
            changed_cursor = false;
        }
        if window.mouse_buttons[0] && window.time - last_click >= wait_between_clicks{
            last_click = window.time;
            if !dragging{
                ray = raycast(view_position, window.camera.get_forward_vec(), 100., vec![test_obj.clone()]);
                if !ray.is_empty() {
                    dragging = true;
            
                    dist = (test_obj.transform.position - view_position).length();
                    println!("Distance: {}", dist);
                }
            }
            else {
                dragging = false;
            }
        }

        if dragging && !ray.is_empty(){
            test_obj.set_position(view_position+dist*window.camera.get_forward_vec());
        }

        test_obj.draw(&view_position);
        hit.draw(&view_position);

        window.update(); 
    }
}
