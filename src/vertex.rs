use glam::{Vec2, Vec3, Vec4};

pub struct Vertex{
    pub position: Vec3,
    pub color: Vec4,
    pub tex_coords: Vec2,
    pub normal: Vec3,
}