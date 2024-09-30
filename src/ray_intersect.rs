// ray_intersect.rs

use crate::material::Material;
use nalgebra::Vector3 as Vec3;
use crate::cube::Cube;
use crate::sphere::Sphere;
use crate::plane::Plane;

#[derive(Debug, Clone)]
pub enum ObjectType {
    Sphere(Sphere),
    Cube(Cube),
    Plane(Plane),
    // Add more object types if needed
}

#[derive(Debug, Clone)]
pub struct Intersect {
    pub point: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub distance: f32,
    pub material: Material,
    pub object_type: ObjectType,
}

impl Intersect {
    pub fn new(
        point: Vec3<f32>,
        normal: Vec3<f32>,
        distance: f32,
        material: Material,
        object_type: ObjectType,
    ) -> Self {
        Intersect {
            point,
            normal,
            distance,
            material,
            object_type,
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3<f32>, ray_direction: &Vec3<f32>) -> Option<Intersect>;
}
