use glam::Vec3;

pub fn lerp(start: Vec3, end: Vec3, t: f32) -> Vec3 {
    start + (end - start) * t
}

pub fn lerpf32(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}
