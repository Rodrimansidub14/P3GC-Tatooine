// render.rs

use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::material::Material;
use crate::plane::Plane;
use crate::ray_intersect::{Intersect, ObjectType, RayIntersect};
use crate::skybox::Skybox;
use crate::sphere::Sphere;
use nalgebra_glm::Vec3;
use rand::random;

const MAX_DEPTH: u32 = 5;

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Sphere],
    cubes: &[Cube],
    planes: &[Plane],
    camera: &Camera,
    suns: &[Sphere],
    lights: &[Light],
    skybox: &Skybox,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = std::f32::consts::PI / 3.0;

    // Precompute camera vectors
    let camera_direction = (camera.target - camera.position).normalize();
    let camera_right = camera_direction.cross(&camera.up).normalize();
    let camera_up = camera_right.cross(&camera_direction).normalize();

    let samples_per_pixel = 4; // Ajusta según sea necesario para anti-aliasing

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let mut pixel_color = Color::black();

            for _ in 0..samples_per_pixel {
                let u = (x as f32 + random::<f32>()) / width;
                let v = (y as f32 + random::<f32>()) / height;

                let i = (2.0 * u - 1.0) * (fov / 2.0).tan() * aspect_ratio;
                let j = (1.0 - 2.0 * v) * (fov / 2.0).tan();

                let ray_direction = (camera_right * i + camera_up * j + camera_direction).normalize();
                let sample_color = cast_ray(
                    &camera.position.coords,
                    &ray_direction,
                    objects,
                    cubes,
                    planes,
                    suns,
                    lights,
                    skybox,
                    0,
                );
                pixel_color = pixel_color + sample_color;
            }

            pixel_color = pixel_color / (samples_per_pixel as f32);
            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x as usize, y as usize);
        }
    }
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    spheres: &[Sphere],
    cubes: &[Cube],
    planes: &[Plane],
    suns: &[Sphere],
    lights: &[Light],
    skybox: &Skybox,
    depth: u32,
) -> Color {
    if depth > MAX_DEPTH {
        return Color::black();
    }

    let mut closest_intersect: Option<Intersect> = None;
    let mut zbuffer = f32::INFINITY;

    // Verificar intersecciones con esferas
    for sphere in spheres.iter().chain(suns.iter()) {
        if let Some(intersect) = sphere.ray_intersect(ray_origin, ray_direction) {
            if intersect.distance < zbuffer {
                zbuffer = intersect.distance;
                closest_intersect = Some(intersect);
            }
        }
    }

    // Verificar intersecciones con cubos
    for cube in cubes {
        if let Some((distance, normal)) = cube.ray_intersect(ray_origin, ray_direction) {
            if distance < zbuffer {
                zbuffer = distance;
                closest_intersect = Some(Intersect::new(
                    ray_origin + ray_direction * distance,
                    normal,
                    distance,
                    cube.material.clone(),
                    ObjectType::Cube(cube.clone()),
                ));
            }
        }
    }

    // Verificar intersecciones con planos
    for plane in planes {
        if let Some(intersect) = plane.ray_intersect(ray_origin, ray_direction) {
            if intersect.distance < zbuffer {
                zbuffer = intersect.distance;
                closest_intersect = Some(intersect);
            }
        }
    }

    if let Some(intersect) = closest_intersect {
        let material = &intersect.material;

        // Obtener el color del material directamente sin texturas
        let material_color = material.color;

        // Iluminación ambiental
        let ambient_intensity = 0.1; // Ajusta este valor según sea necesario
        let ambient_color = material_color * ambient_intensity;

        // Inicializar colores difuso y especular
        let mut diffuse_color = Color::black();
        let mut specular_color = Color::black();

        for light in lights {
            let light_dir = (light.position - intersect.point).normalize();
            let light_distance = (light.position - intersect.point).magnitude();

            // Comprobación de sombras
            let shadow_bias = intersect.normal * 1e-5;
            let shadow_origin = if light_dir.dot(&intersect.normal) < 0.0 {
                intersect.point - shadow_bias
            } else {
                intersect.point + shadow_bias
            };
            let mut shadow_intersect = false;

            // Comprobar sombras con esferas
            for sphere in spheres.iter().chain(suns.iter()) {
                if let Some(shadow) = sphere.ray_intersect(&shadow_origin, &light_dir) {
                    if shadow.distance < light_distance {
                        shadow_intersect = true;
                        break;
                    }
                }
            }

            // Comprobar sombras con cubos
            for cube in cubes {
                if let Some((shadow_distance, _)) = cube.ray_intersect(&shadow_origin, &light_dir) {
                    if shadow_distance < light_distance {
                        shadow_intersect = true;
                        break;
                    }
                }
            }

            // Comprobar sombras con planos
            for plane in planes {
                if let Some(shadow) = plane.ray_intersect(&shadow_origin, &light_dir) {
                    if shadow.distance < light_distance {
                        shadow_intersect = true;
                        break;
                    }
                }
            }

            if !shadow_intersect {
                // Iluminación difusa
                let diffuse_intensity = light_dir.dot(&intersect.normal).max(0.0) * light.intensity;
                diffuse_color = diffuse_color + (material_color * diffuse_intensity * material.albedo[0]);

                // Iluminación especular
                let reflect_dir = reflect(&(-light_dir), &intersect.normal);
                let specular_intensity = reflect_dir
                    .dot(&-ray_direction)
                    .max(0.0)
                    .powf(material.specular)
                    * light.intensity;
                specular_color = specular_color
                    + (Color::new(255, 255, 255) * specular_intensity * material.albedo[1]);
            }
        }

        // Reflexión y refracción
        let mut reflectivity = material.albedo[2];
        let mut transparency = material.albedo[3];

        // Normalizar reflectividad y transparencia si su suma es mayor que 1.0
        let total_reflect_transp = reflectivity + transparency;
        if total_reflect_transp > 1.0 {
            reflectivity /= total_reflect_transp;
            transparency /= total_reflect_transp;
        }

        let mut reflection_color = Color::black();
        if reflectivity > 0.0 && depth < MAX_DEPTH {
            let reflect_dir = reflect(ray_direction, &intersect.normal).normalize();
            let reflect_origin = if reflect_dir.dot(&intersect.normal) < 0.0 {
                intersect.point - intersect.normal * 1e-3
            } else {
                intersect.point + intersect.normal * 1e-3
            };
            reflection_color = cast_ray(
                &reflect_origin,
                &reflect_dir,
                spheres,
                cubes,
                planes,
                suns,
                lights,
                skybox,
                depth + 1,
            );
        }

        let mut refraction_color = Color::black();
        if transparency > 0.0 && depth < MAX_DEPTH {
            let eta = if ray_direction.dot(&intersect.normal) < 0.0 {
                1.0 / material.refractive_index
            } else {
                material.refractive_index
            };
            if let Some(refract_dir) = refract(ray_direction, &intersect.normal, eta) {
                let refract_origin = if refract_dir.dot(&intersect.normal) < 0.0 {
                    intersect.point - intersect.normal * 1e-3
                } else {
                    intersect.point + intersect.normal * 1e-3
                };
                refraction_color = cast_ray(
                    &refract_origin,
                    &refract_dir.normalize(),
                    spheres,
                    cubes,
                    planes,
                    suns,
                    lights,
                    skybox,
                    depth + 1,
                );
            }
        }

        // Calcular el color final
        let mut final_color = ambient_color + diffuse_color + specular_color;

        // Aplicar efectos de reflexión y refracción
        final_color = final_color * (1.0 - reflectivity - transparency)
            + reflection_color * reflectivity
            + refraction_color * transparency
            + material.emissive;

        // Asegurar que el color final está dentro de los límites
        final_color.clamp()
    } else {
        // No hay intersección, retorna el color del skybox
        skybox.get_color(ray_direction)
    }
}

fn reflect(direction: &Vec3, normal: &Vec3) -> Vec3 {
    direction - normal * 2.0 * direction.dot(normal)
}

fn refract(direction: &Vec3, normal: &Vec3, eta: f32) -> Option<Vec3> {
    let cosi = -direction.dot(normal).clamp(-1.0, 1.0);
    let etai = 1.0;
    let etat = eta;
    let mut n = normal.clone();
    let mut eta = etai / etat;
    if cosi < 0.0 {
        eta = etat / etai;
        n = -normal;
    }
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        None
    } else {
        Some(direction * eta + n * (eta * cosi - k.sqrt()))
    }
}
