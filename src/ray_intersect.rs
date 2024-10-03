// ray_intersect.rs

use crate::material::Material;
use nalgebra::Vector3 as Vec3;

#[derive(Debug, Clone)]
pub struct Intersect {
    pub point: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vec3<f32>, normal: Vec3<f32>, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
        }
    }
}
// Define the RayIntersect trait
pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3<f32>, ray_direction: &Vec3<f32>) -> Option<Intersect>;
}
