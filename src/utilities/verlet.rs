use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Verlet {
    pub mass: f32,
    pub position: Vec3,
    pub prev_position: Vec3,
    pub vel: Vec3,
    acceleration: Vec3,
    pub force: Vec3,
}

impl Verlet {
    pub fn new(mass: f32, position: Option<Vec3>) -> Self {
        let pos = position.unwrap_or(Vec3::ZERO);
        Self {
            mass,
            position: pos,
            prev_position: pos,
            vel: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            force: Vec3::ZERO,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.acceleration = self.force / self.mass;

        // Verlet integration
        self.position += self.vel * dt + 0.5 * self.acceleration * dt * dt;

        let new_acceleration = self.force / self.mass;

        // Update velocity with average acceleration
        self.vel += 0.5 * (self.acceleration + new_acceleration) * dt;

        // Damping for stability
        self.vel *= 0.98;

        self.force = Vec3::ZERO;
    }
}
