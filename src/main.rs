use core::f32;
use std::{cell::RefCell, collections::HashMap, rc::Rc, thread, time::Duration};

use rand::Rng;
use reng::game_object::GameObject;
use glam::{vec2, vec3, vec4, Quat, Vec2, Vec3, Vec4};

use glfw::{Action, Key};
use reng::light::{Light, LIGHTS};
use reng::line::Line;
use reng::mesh::Mesh;
use reng::{load_model, quickGO, Transform};
use reng::shapes::Shapes;
use reng::texture::make_tex;
use reng::window::Window;
use reng::shader::Shader;
use reng::instance_rendering::create_instance_buffer;

// settings
const W: u32 = 800;
const H: u32 = 600;
const G: Vec3 = vec3(0., -9.81, 0.);

pub fn main() {
    let mut window = Window::new(W, H);
    window.set_caption("ulala babe babe cmon");

    let mut texture_pack: HashMap<usize, u32> = HashMap::default();
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));
    texture_pack.insert(2, make_tex("src/textures/container.jpg"));
    // texture_pack.insert(3, make_tex("src/textures/grass_sprite.png"));
    // texture_pack.insert(4, make_tex("src/models/revolver/revolver.png"));
    // texture_pack.insert(5, make_tex("src/models/soda/soda.png"));
    // texture_pack.insert(6, make_tex("src/models/tetra/tetra.png"));

    unsafe{
        LIGHTS.push(Light { position: vec3(0., 0., 0.), color: vec3(1., 1., 1.)/3. });
    }

    let mut world_borders = vec![
        GameObject::<Vec<Mesh>>::new(Mesh::empty()),
        GameObject::<Vec<Mesh>>::new(Mesh::empty()),
        GameObject::<Vec<Mesh>>::new(Mesh::empty()),
        GameObject::<Vec<Mesh>>::new(Mesh::empty()),
        GameObject::<Vec<Mesh>>::new(Mesh::empty()),
        GameObject::<Vec<Mesh>>::new(Mesh::empty()),
    ];

    for wall in world_borders.iter_mut(){
        wall.set_shape(Shapes::Cube);
        wall.set_texture(texture_pack[&1]);
        wall.set_color(vec4(1., 0., 0.7, 0.2));
        wall.setup_meshes();
    }

    // setting up world borders for visual purposes
    world_borders[0].scale3D(vec3(20., 0.1, 20.));
    world_borders[0].translate(vec3(0., -10., 0.));
    world_borders[1].scale3D(vec3(20., 0.1, 20.));
    world_borders[1].translate(vec3(0., 10., 0.));
    world_borders[2].scale3D(vec3(0.1, 20., 20.));
    world_borders[2].translate(vec3(10., 0., 0.));
    world_borders[3].scale3D(vec3(0.1, 20., 20.));
    world_borders[3].translate(vec3(-10., 0., 0.));
    world_borders[4].scale3D(vec3(20., 20., 0.1));
    world_borders[4].translate(vec3(0., 0., 10.));
    world_borders[5].scale3D(vec3(20., 20., 0.1));
    world_borders[5].translate(vec3(0., 0., -10.));

    let mut changed_cursor = false;

    let mut world = World::new();

    let mut rng = rand::thread_rng();
    
    window.lock_cursor();
    while !window.should_close() {
        let view_position = window.camera.position;
        
        unsafe{
            LIGHTS[0].position = view_position;
        }

        window.camera.movement(window.keyboard.clone(), window.dt);

        if window.keyboard[&Key::LeftAlt] == Action::Press {
            if !changed_cursor{
                window.lock_cursor();
                changed_cursor = true;
            }
        }
        else if window.keyboard[&Key::LeftAlt] == Action::Release {
            changed_cursor = false;
        }

        if window.keyboard[&Key::Right] != Action::Release{
            world.particles[0].apply_force(vec3(50., 0., 0.));
        }
        if window.keyboard[&Key::Left] != Action::Release{
            world.particles[0].apply_force(vec3(-50., 0., 0.));
        }
        if window.keyboard[&Key::Up] != Action::Release{
            world.particles[0].apply_force(vec3(0., 50., 0.));
        }
        if window.keyboard[&Key::Down] != Action::Release{
            world.particles[0].apply_force(vec3(0., -50., 0.));
        }
        if window.keyboard[&Key::K] != Action::Release{
            world.particles[0].apply_force(vec3(0., 0., 50.));
        }
        if window.keyboard[&Key::I] != Action::Release{
            world.particles[0].apply_force(vec3(0., 0., -50.));
        }
        if window.keyboard[&Key::Q] != Action::Release{
            let mut particle = quickGO(Shapes::Sphere, texture_pack[&1]);
            particle.set_position(vec3(
                -10.+rng.gen_range(0.0..21.0),
                -10.+rng.gen_range(0.0..21.0),
                -10.+rng.gen_range(0.0..21.0)
            ));
            // my instancing can't take random colors 😿
            // particle.set_color(vec4(
            //     rng.gen_range(0.0..256.0) / 256.,
            //     rng.gen_range(0.0..256.0) / 256.,
            //     rng.gen_range(0.0..256.0) / 256.,
            //     1.
            // ));
            particle.scale(0.5);
            particle.set_shader("src/shaders/default_lit_shader.vs", "src/shaders/default_lit_shader.fs");
            particle.setup_meshes();

            world.particles.push(Particle::new(particle));
        }

        window.clear_screen();
        
        world.update(window.dt);
        world.handle_collisions();
        world.draw(&view_position);

        for wall in world_borders.iter_mut(){
            wall.draw(&view_position);
        }

        window.update();
    }
}


struct World {
    pub particles: Vec<Particle>,
}

impl World{
    pub fn new() -> Self{
        World{
            particles: vec![],
        }
    }

    pub fn update(&mut self, dt: f32){
        for particle in self.particles.iter_mut(){
            particle.apply_force(G);
            particle.update(dt);
            particle.constraint();
        }
    }

    pub fn handle_collisions(&mut self) {
        let num_particles = self.particles.len();
        for i in 0..num_particles {
            for j in (i + 1)..num_particles {
                let (p1, p2) = self.particles.split_at_mut(j);
                let p1 = &mut p1[i];
                let p2 = &mut p2[0];
                let r1 = p1.obj.transform.scale.x;
                let r2 = p2.obj.transform.scale.x;

                let delta = p2.obj.transform.position - p1.obj.transform.position;
                let distance = delta.length();
                let min_distance = r1 + r2;

                if distance < min_distance {
                    // Resolve collision
                    let overlap = min_distance - distance;
                    let correction = delta.normalize() * (overlap / 2.0);

                    p1.obj.transform.position -= correction;
                    p2.obj.transform.position += correction;

                    // Adjust velocities
                    let relative_velocity = p2.velocity - p1.velocity;
                    let normal = delta.normalize();
                    let velocity_along_normal = relative_velocity.dot(normal);

                    if velocity_along_normal > 0.0 {
                        continue;
                    }

                    let restitution = 0.5;  // Coefficient of restitution (elasticity)
                    let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
                    let impulse = impulse_scalar * normal;

                    let total_mass = p1.obj.transform.scale.x + p2.obj.transform.scale.x;

                    p1.velocity -= impulse/(total_mass/p1.obj.transform.scale.x);
                    p2.velocity += impulse/(total_mass/p2.obj.transform.scale.x);
                }
            }
        }
    }

    pub fn draw(&mut self, view_position: &Vec3){
        for particle in self.particles.iter_mut(){
            particle.obj.draw(view_position);
        }
        // if self.particles.len() > 0 {
            
        //     let instance_data: Vec<Transform> = self.particles.iter().map(|particle| {
        //     Transform {
        //         position: particle.obj.transform.position,
        //         scale: particle.obj.transform.scale,
        //         rotation: particle.obj.transform.rotation,
        //     }
        //     }).collect();

        //     for GO in self.particles.iter_mut(){
        //         GO.obj.bind_instanced(*view_position);
        //     }

        //     let instance_buffer = create_instance_buffer(&instance_data);
        //     let instance_count = self.particles.len() as i32;

        //     GameObject::draw_instanced(instance_buffer, instance_count, self.particles[0].obj.object[0].indices.len());
        // }
    }
}


struct Particle{
    pub obj: GameObject::<Vec<Mesh>>,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub force: Vec3,
}

impl Particle{
    pub fn new(obj: GameObject::<Vec<Mesh>>) -> Self{
        Particle{
            obj,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            force: Vec3::ZERO,
        }
    }

    pub fn apply_force(&mut self, force: Vec3){
        self.force += force;
    }

    pub fn update(&mut self, dt: f32){
        self.acceleration = (self.force)/(self.obj.transform.scale.x*100.);
        self.velocity += self.acceleration;
        self.obj.translate(self.velocity*dt);
        self.force = Vec3::ZERO;
    }

    pub fn constraint(&mut self){
        let mut pos = self.obj.transform.position;
        
        if pos.x < -10.+self.obj.transform.scale.x{
            pos.x = -10.+self.obj.transform.scale.x;
            self.velocity.x = -self.velocity.x;
        }
        if pos.x > 10.-self.obj.transform.scale.x{
            pos.x = 10.-self.obj.transform.scale.x;
            self.velocity.x = -self.velocity.x;
        }
        if pos.y < -10.+self.obj.transform.scale.x{
            pos.y = -10.+self.obj.transform.scale.x;
            self.velocity.y = -self.velocity.y;
        }
        if pos.y > 10.-self.obj.transform.scale.x{
            pos.y = 10.-self.obj.transform.scale.x;
            self.velocity.y = -self.velocity.y;
        }
        if pos.z < -10.+self.obj.transform.scale.x{
            pos.z = -10.+self.obj.transform.scale.x;
            self.velocity.z = -self.velocity.z;
        }
        if pos.z > 10.-self.obj.transform.scale.x{
            pos.z = 10.-self.obj.transform.scale.x;
            self.velocity.z = -self.velocity.z;
        }
        
        self.obj.set_position(pos);
    }
}
