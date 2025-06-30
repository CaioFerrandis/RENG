use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Spring{
    pub p1: Vec3,
    pub p2:Vec3,
    pub size: f32,
    pub strength: f32, // can range form 0.0 to 1.0
}

impl Spring{
    pub fn new(p1: Vec3, p2: Vec3, size: f32, spring_strength: f32) -> Self{
        let mut strength = spring_strength;
        if strength < 0. {
            strength = 0.
        }
        else if strength > 1.{
            strength = 1.;
        }

        Self{
            p1,
            p2,
            size,
            strength,
        }
    }

    pub fn get_force(&self, k: f32) -> Vec3 {
    let dir = self.p2 - self.p1;
    let dist = dir.length();
    let offset = dist - self.size;
    if dist != 0.0 {
        -k * offset * dir.normalize()
    } else {
        Vec3::ZERO
    }
}

    pub fn update(&mut self){
        let dir = self.p2 - self.p1;
        let dist = dir.length();
        let offset = dist - self.size;

        if dist != 0.0 {
            let correction = dir.normalize() * (offset / 2.0) * self.strength;
            self.p1 += correction;
            self.p2 -= correction;
        }
    }
}
