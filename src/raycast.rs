use core::f32;

use glam::Vec3;

use crate::{GameObject, mesh::Mesh, Shapes};

pub fn raycast(
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    objects: Vec<&GameObject<Mesh>>,
) -> Vec<(String, f32, Vec3)> {
    let mut collisions = Vec::new();
    
    for object in objects {
        match object.shape {
            Shapes::Sphere => {
                if let Some(distance) = ray_sphere_intersection(origin, direction, object.transform.position, object.transform.scale.x) {
                    if distance <= max_distance {
                        let hit_point = origin + direction * distance;
                        collisions.push((object.clone().name, distance, hit_point));
                    }
                }
            }
            Shapes::Cube => {
                // Compute the cube's min and max points
                let half_scale = object.transform.scale / 2.0;
                let cube_min = object.transform.position - half_scale;
                let cube_max = object.transform.position + half_scale;
            
                // Perform the ray-cube intersection test
                if let Some(distance) = ray_cube_intersection(origin, direction, cube_min, cube_max) {
                    if distance <= max_distance {
                        let hit_point = origin + direction * distance;
                        collisions.push((object.clone().name, distance, hit_point));
                    }
                }
            }
            _ => {}
        }
    }

    collisions
}

pub fn closest_raycast(
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    objects: Vec<&GameObject<Mesh>>,
) -> Option<(String, f32, Vec3)> {
    let hits = raycast(origin, direction, max_distance, objects);

    let mut closest: Option<(String, f32, Vec3)> = None;

    for hit in hits{
        if !closest.is_some() || closest.clone().unwrap().1 > hit.1{
            closest = Some(hit);
        }
    }
    closest
}

fn ray_sphere_intersection(
    ray_origin: Vec3,
    ray_direction: Vec3,
    sphere_center: Vec3,
    sphere_radius: f32,
) -> Option<f32> {
    let oc = ray_origin - sphere_center;
    let a = ray_direction.dot(ray_direction);
    let b = 2.0 * oc.dot(ray_direction);
    let c = oc.dot(oc) - sphere_radius * sphere_radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 >= 0.0 {
            Some(t1)
        } else if t2 >= 0.0 {
            Some(t2)
        } else {
            None
        }
    }
}

fn ray_cube_intersection(
    ray_origin: Vec3,
    ray_direction: Vec3,
    cube_min: Vec3,
    cube_max: Vec3,
) -> Option<f32> {
    let inv_dir = 1.0 / ray_direction; // Handle division by direction

    let tmin = (cube_min - ray_origin) * inv_dir;
    let tmax = (cube_max - ray_origin) * inv_dir;

    let t1 = tmin.min(tmax); // Entry point per axis
    let t2 = tmin.max(tmax); // Exit point per axis

    let t_near = t1.max_element(); // Furthest entry point
    let t_far = t2.min_element();  // Closest exit point

    if t_near <= t_far && t_far >= 0.0 {
        Some(t_near) // Return the closest intersection distance
    } else {
        None // No intersection
    }
}
