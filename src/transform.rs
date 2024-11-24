use glam::{Quat, Vec3};

#[derive(Copy, Clone)]
pub struct Transform{
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}

impl Transform{
    pub fn new() -> Self{
        Transform{
            position: Vec3::ZERO,
            scale: Vec3::ONE,
            rotation: Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0),
        }
    }
}
