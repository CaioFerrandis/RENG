use core::f32;
use std::collections::HashMap;

use collision::Volume;
use glam::{vec3, vec4, Vec3};
use glfw::{Key, Action};
use reng::*;
use rapier3d::prelude::*;

fn main() {
    // set up physics stuff
    let mut physics_pipeline = PhysicsPipeline::new();
    let gravity = vector![0.0, -9.81, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut narrow_phase = NarrowPhase::new();
    let mut island_manager = IslandManager::new();
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    // --------------------

    let mut window = Window::new(800, 600);
    window.set_caption("It's time for some slootin' shootin'");
    
    let mut texture_pack: HashMap<usize, u32> = HashMap::default();
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));
    texture_pack.insert(2, make_tex("src/textures/grass.jpg"));

    unsafe{
        LIGHTS.push(Light { position: vec3(0., 0., 0.), color: vec3(1., 1., 1.)/3. });
    }

    let mut world = Vec::<GameObject::<Mesh>>::new();

    let mut floor = GameObject::<Mesh>::new(Mesh::empty());
    floor.set_shape(Shapes::Cube);
    floor.set_texture(texture_pack[&2]); // grass.jpg
    floor.scale3D(vec3(1000., 10., 1000.));
    floor.transform.position.y -= 7.;
    floor.setup_mesh();

    let floor_collider = ColliderBuilder::cuboid(5000., 5., 5000.).build();
    collider_set.insert(floor_collider);

    world.push(floor);

    let mut bullets = Vec::<Bullet>::new();

    let mut enemies = Vec::<Enemy>::new();

    let mut changed_cursor = true;

    let mut enemy_last_spawn = window.time;
    let enemy_spawn_time = 1.;

    while !window.should_close() {
        /* main game loop */
        unsafe{
            LIGHTS[0].position = window.camera.position;
        }

        let view_position = window.camera.position;

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

        /* */
        if window.mouse_buttons[0]{
            let view_matrix = window.camera.get_view_matrix();
            
            let forward = -glam::Vec3::new(view_matrix.x_axis.z, view_matrix.y_axis.z, view_matrix.z_axis.z);
            let mut object = GameObject::<Mesh>::new(Mesh::empty());
            
            object.transform.position += window.camera.right * 2.0 - 2.0 * window.camera.up;

            object.scale(0.3);
            object.set_shape(Shapes::Sphere);
            object.set_color(vec4(1., 0., 0., 1.));
            object.set_texture(texture_pack[&1]); // default_tex.png
            object.setup_mesh();
            object.set_position(view_position);


            let bullet = Bullet::new(forward, object, 50.0);

            bullets.push(bullet);
        }

        /* update the bullets */
        let mut bullets_to_die = vec![];
        for (i, bullet) in bullets.iter_mut().enumerate() {
            bullet.update_bullet(&world, &window);

            if bullet.should_die {
                bullets_to_die.push(i);
            }
        }
        for idx in bullets_to_die {
            if idx < bullets.len() {
                bullets[idx].object.object.destroy();
                bullets.remove(idx);
            }
        }

        /* ocasionally spawn the enemies */
        if window.time - enemy_last_spawn >= enemy_spawn_time {
            enemy_last_spawn = window.time;
            let mut object = GameObject::<Mesh>::new(Mesh::empty());
            
            object.scale(1.5);
            object.set_shape(Shapes::Cube);
            object.set_color(vec4(0., 1., 1., 0.5));
            object.set_texture(texture_pack[&1]); // default_tex.png
            object.setup_mesh();
            object.set_position(view_position + window.camera.front * 20.0 + vec3(0., 5., 0.));

            let rigid_body = RigidBodyBuilder::dynamic().translation(vector![0., 5., 0.]).build();
            let cube_collider = ColliderBuilder::cuboid(0.75, 0.75, 0.75).build();
            let body_handle = rigid_body_set.insert(rigid_body);
            collider_set.insert_with_parent(cube_collider, body_handle, &mut rigid_body_set);
            

            enemies.push(Enemy::new(100.0, object, 0.5));
        }

        /* update the enemies */
        let mut enemies_to_kill = vec![];
        for (i, enemy) in enemies.iter_mut().enumerate() {
            enemy.update(&world, &window, &mut bullets);
            if enemy.dead {
                enemies_to_kill.push(i);
            }
        }

        for idx in enemies_to_kill {
            if idx < enemies.len() {
                enemies[idx].object.object.destroy();
                enemies.remove(idx);
            }
        }

        for object in world.iter_mut(){
            object.draw(&view_position);
        }

        window.update(); 
    }
}

struct Bullet{
    pub forward: Vec3,
    pub object: GameObject<Mesh>,
    pub speed: f32,
    should_die: bool,
    pub max_duration: f32,
    time: f32,
}

impl Bullet{
    pub fn new(forward: Vec3, object: GameObject<Mesh>, speed: f32) -> Self{
        Self{
            forward,
            object,
            speed,
            should_die: false,
            max_duration: 10.0, // temporary value
            time: 0.0,
        }
    }

    pub fn update_bullet(&mut self, world: &Vec<GameObject<Mesh>>, window: &Window) {
        self.time += window.dt;
        if self.time >= self.max_duration{
            self.should_die = true;
        }
        
        let collide_pos = self.object.transform.position;

        self.object.transform.position += self.forward * window.dt * self.speed;
    
        for object in world.iter(){
            if object.collide(collide_pos){
                self.should_die = true;
            }
        }

        self.object.draw(&window.camera.position);
    }
}

struct Enemy{
    life: f32,
    object: GameObject<Mesh>,
    dead: bool,
    speed: f32,
}

impl Enemy{
    pub fn new(life: f32, object: GameObject<Mesh>, speed: f32) -> Self{
        Self { life, object, dead: false, speed }
    }

    pub fn update(&mut self, world: &Vec<GameObject<Mesh>>, window: &Window, bullets: &mut Vec<Bullet>){
        if self.life <= 0.{
            self.dead = true;
        }

        let mut collide_pos = self.object.transform.position;
        collide_pos.y -= self.object.transform.scale.x;

        let mut move_dir = self.object.transform.position - window.camera.position;
        move_dir.y = 0.;
        
        for object in world.iter(){
            if object.collide(collide_pos){
                self.object.transform.position -= move_dir*self.speed*window.dt;
            }
            else{
                self.object.transform.position.y -= window.dt*G;
            }
        }

        for bullet in bullets.iter_mut(){
            let p = bullet.object.transform.position;
            if self.object.collide(p) {
                self.life -= 5.0;
                bullet.should_die = true;
            }
        }

        self.object.draw(&window.camera.position);
    }
}
