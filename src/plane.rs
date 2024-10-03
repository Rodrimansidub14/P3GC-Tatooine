use nalgebra::Vector3 as Vec3;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3<f32>,    // Un punto en el plano
    pub normal: Vec3<f32>,   // Vector normal del plano
    pub material: Material,  // Material del plano, que puede tener textura
}

impl Plane {
    // Constructor para un nuevo plano
    pub fn new(point: Vec3<f32>, normal: Vec3<f32>, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }

    // Método para obtener el color en un punto, considerando la textura
    pub fn get_color_at(&self, point: &Vec3<f32>) -> Color {
        if let Some(texture) = &self.material.texture {
            // Mapeo UV básico
            let u = (point.x - self.point.x).abs() % 1.0; // Ajusta según el tamaño de la textura
            let v = (point.z - self.point.z).abs() % 1.0;
            let x = (u * (texture.width as f32)) as usize;
            let y = (v * (texture.height as f32)) as usize;
            texture.get_color(x, y)
        } else {
            // Si no hay textura, usar el color base
            self.material.color
        }
    }
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ray_origin: &Vec3<f32>, ray_direction: &Vec3<f32>) -> Option<Intersect> {
        let denom = self.normal.dot(ray_direction);

        if denom.abs() > 1e-6 {
            let t = (self.point - ray_origin).dot(&self.normal) / denom;
            if t >= 0.0 {
                let hit_point = ray_origin + ray_direction * t;

                // Check if the hit point is within the finite bounds of the plane (size 10)
                if hit_point.x.abs() > 5.0 || hit_point.z.abs() > 5.0 {
                    return None;  // The intersection is outside the plane's bounds
                }

                // Calculate texture coordinates (u, v) for the normal map
                // Assuming the plane is mapped from -5 to 5 in world space to 0 to 1 in texture space
                let u = (hit_point.x + 5.0) / 10.0;
                let v = (hit_point.z + 5.0) / 10.0;

                // Perturb the normal using the normal map if it exists
                let mut normal = self.normal;
                if let Some(normal_map) = &self.material.normal_map {
                    let normal_color = normal_map.get_color((u * (normal_map.width - 1) as f32) as usize, (v * (normal_map.height - 1) as f32) as usize);
                    
                    // Convert the normal color from RGB to a perturbation vector
                    let perturbation = Vec3::new(
                        normal_color.r as f32 / 255.0 * 2.0 - 1.0,
                        normal_color.g as f32 / 255.0 * 2.0 - 1.0,
                        normal_color.b as f32 / 255.0 * 2.0 - 1.0
                    );
                    normal = (normal + perturbation).normalize();  // Apply the normal perturbation
                }

                return Some(Intersect::new(hit_point, normal, t, self.material.clone()));
            }
        }
        None
    }
}

