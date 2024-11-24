use glam::Vec3;

pub static mut LIGHTS: Vec<Light> = vec![];

pub struct Light{
    pub position: Vec3,
    pub color: Vec3,
}
