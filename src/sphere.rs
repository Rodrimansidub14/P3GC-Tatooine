// sphere.rs

use nalgebra_glm::{Vec3, dot};
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect, ObjectType};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    // Método para calcular las coordenadas UV para texturizado (puedes eliminarlo)
    /*
    pub fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let hit_point = (point - self.center).normalize();
        let u = 0.5 + hit_point.z.atan2(hit_point.x) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - hit_point.y.asin() / std::f32::consts::PI;
        (u, v)
    }
    */
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b - sqrt_disc) / (2.0 * a);
            let t2 = (-b + sqrt_disc) / (2.0 * a);

            let t = if t1 > 0.0 {
                t1
            } else if t2 > 0.0 {
                t2
            } else {
                return None;
            };

            let point = ray_origin + ray_direction * t;
            let normal = (point - self.center).normalize();
            let distance = t;

            Some(Intersect::new(
                point,
                normal,
                distance,
                self.material.clone(),
                ObjectType::Sphere(self.clone()),
            ))
        } else {
            None
        }
    }
}
