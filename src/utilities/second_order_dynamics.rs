use glam::Vec3;

#[derive(Clone, Copy)]
pub struct SecondOrderDynamics {
    xp: Vec3,
    pub y: Vec3,
    yd: Vec3,
    k1: f32,
    k2: f32,
    k3: f32,
    pub force: Vec3
}

impl SecondOrderDynamics {
    pub fn new(frequency: f32, damping: f32, response_time: f32, pos0: Vec3) -> Self { // f: frequency, z: damping ratio (z = 0 system undamped; 0 < z < 1, system vibrates to stability, z > 1 system doesn't vibrate), r: response time (r = 0 system takes time to accelerate; r = 1 system reacts instantaneously; r > 1 system overshoots target; r < 0 system antecipates motion)
        let f = frequency;
        let z = damping;
        let r = response_time;

        let pi = std::f32::consts::PI;
        let k1 = z / (pi * f);
        let k2 = 1.0 / ((2.0 * pi * f) * (2.0 * pi * f));
        let k3 = r * z / (2.0 * pi * f);

        Self {
            xp: pos0,
            y: pos0,
            yd: Vec3::ZERO,
            k1,
            k2,
            k3,
            force: Vec3::ZERO,
        }
    }

    pub fn update(&mut self, t: f32, pos: Vec3, vel: Option<Vec3>) -> Vec3 {
        if t == 0. || !t.is_finite(){ return self.y }

        let xd = vel.unwrap_or((pos - self.xp) / t);
        self.xp = pos;

        let k2_stable = self.k2.max(t * t / 2.0 + t * self.k1 / 2.0);
        
        self.y += t * self.yd;
        self.yd += t * ((pos + self.k3 * xd - self.y - self.k1 * self.yd) / k2_stable + self.force);

        self.force = Vec3::ZERO;

        self.y
    }
}
