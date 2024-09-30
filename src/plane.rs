// plane.rs

use nalgebra::{Vector3 as Vec3};
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3<f32>,    // A point on the plane
    pub normal: Vec3<f32>,   // The normal vector of the plane
    pub material: Material,  // Material of the plane
}

impl Plane {
    // Constructor for a new plane
    pub fn new(point: Vec3<f32>, normal: Vec3<f32>, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }

    // Método para calcular las coordenadas UV para texturizado (puedes eliminarlo)
    /*
    pub fn get_uv(&self, point: &Vec3<f32>) -> (f32, f32) {
        let scale = 0.1; // Ajusta según sea necesario
        let u = point.x * scale;
        let v = point.z * scale;
        (u.fract(), v.fract())
    }
    */
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ray_origin: &Vec3<f32>, ray_direction: &Vec3<f32>) -> Option<Intersect> {
        let denom = self.normal.dot(ray_direction);

        if denom.abs() > 1e-6 {
            let t = (self.point - ray_origin).dot(&self.normal) / denom;

            if t >= 0.0 {
                let hit_point = ray_origin + ray_direction * t;

                // Opcionalmente, limita el plano a un tamaño finito
                // Para un plano infinito, puedes omitir esta verificación
                let plane_size = 50.0; // Tamaño de ejemplo
                if hit_point.x.abs() > plane_size || hit_point.z.abs() > plane_size {
                    return None;
                }

                return Some(Intersect::new(
                    hit_point,
                    self.normal,
                    t,
                    self.material.clone(),
                    crate::ray_intersect::ObjectType::Plane(self.clone()),
                ));
            }
        }
        None
    }
}
