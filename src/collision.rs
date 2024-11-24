pub use glam::*;

use crate::{get_model_matrix, GameObject, Mesh, Shapes};

pub trait Volume {
    fn collide(&self, p: Vec3,) -> bool;
}

impl Volume for GameObject<Mesh> {
    fn collide(&self, p: Vec3) -> bool {
        match self.shape {
            Shapes::Empty => {
                return false;
            },
            Shapes::Circle => todo!(),
            Shapes::Quad => todo!(),
            Shapes::Cube => {
                let inv_model_matrix = get_model_matrix(self.transform).inverse();
                // Transform the point into the cube's local space
                let local_point = inv_model_matrix.transform_point3(p);
        
                // Check if the local point lies within the unit cube boundaries
                if local_point.x >= -0.5 && local_point.x <= 0.5 &&
                local_point.y >= -0.5 && local_point.y <= 0.5 &&
                local_point.z >= -0.5 && local_point.z <= 0.5 { true } else { false } // god has abandoned us
            },
            Shapes::Sphere => {
                let center = self.transform.position;
                let radius = self.transform.scale.x;

                if p.distance(center) <= radius {
                    return true;
                } else {
                    return false;
                }
                
            },
            Shapes::Triangle => todo!(),
            Shapes::Line => todo!(),
        }
    }
}
