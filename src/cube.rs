use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use nalgebra::{Point3, Vector3 as Vec3};

#[derive(Debug, Clone)]
pub struct Cube {
    pub center: Point3<f32>,
    pub size: f32,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Point3<f32>, size: f32, material: Material) -> Self {
        Cube {
            center,
            size,
            material,
        }
    }

    // Method to compute UV coordinates for texturing
    pub fn get_uv(&self, point: &Vec3<f32>, normal: &Vec3<f32>) -> (f32, f32) {
        let half_size = self.size / 2.0;
        let local_point = point - self.center.coords;
        let (u, v) = if normal.x.abs() > 0.99 {
            // Faces on X axis
            (
                (local_point.z + half_size) / self.size,
                (local_point.y + half_size) / self.size,
            )
        } else if normal.y.abs() > 0.99 {
            // Faces on Y axis
            (
                (local_point.x + half_size) / self.size,
                (local_point.z + half_size) / self.size,
            )
        } else {
            // Faces on Z axis
            (
                (local_point.x + half_size) / self.size,
                (local_point.y + half_size) / self.size,
            )
        };
        (u.fract(), v.fract()) // Ensure the coordinates are between 0 and 1 (texture wrapping)
    }
}

// Implementación del trait RayIntersect para la estructura Cube
impl RayIntersect for Cube {
    fn ray_intersect(
        &self,
        ray_origin: &Vec3<f32>,
        ray_direction: &Vec3<f32>,
    ) -> Option<Intersect> {
        let half_size = self.size / 2.0;
        let min_bound = self.center.coords - Vec3::new(half_size, half_size, half_size);
        let max_bound = self.center.coords + Vec3::new(half_size, half_size, half_size);

        let dir_fraction = Vec3::new(
            if ray_direction.x != 0.0 {
                1.0 / ray_direction.x
            } else {
                f32::INFINITY
            },
            if ray_direction.y != 0.0 {
                1.0 / ray_direction.y
            } else {
                f32::INFINITY
            },
            if ray_direction.z != 0.0 {
                1.0 / ray_direction.z
            } else {
                f32::INFINITY
            },
        );

        let t1 = (min_bound.x - ray_origin.x) * dir_fraction.x;
        let t2 = (max_bound.x - ray_origin.x) * dir_fraction.x;
        let t3 = (min_bound.y - ray_origin.y) * dir_fraction.y;
        let t4 = (max_bound.y - ray_origin.y) * dir_fraction.y;
        let t5 = (min_bound.z - ray_origin.z) * dir_fraction.z;
        let t6 = (max_bound.z - ray_origin.z) * dir_fraction.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return None;
        }

        let t = if tmin < 0.0 { tmax } else { tmin };
        let hit_point = ray_origin + ray_direction * t;
        let normal = (hit_point - self.center.coords).normalize();

        // Calculate UV coordinates for cube faces
        let (u, v) = self.get_uv(&hit_point, &normal);

        // Get texture color if available
        let color = if let Some(texture) = &self.material.texture {
            let u_pixel = (u * (texture.width - 1) as f32).round() as usize;
            let v_pixel = (v * (texture.height - 1) as f32).round() as usize;
            texture.as_ref().get_color(u_pixel, v_pixel) // Dereference Arc to get the inner Texture
        } else {
            self.material.color
        };
        

        // Return the intersect object with the texture color if available
        Some(Intersect::new(hit_point, normal, t, Material {
            color, // Use the color obtained from the texture or default material color
            ..self.material.clone()
        }))
    }
}
