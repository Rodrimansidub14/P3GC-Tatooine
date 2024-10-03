use nalgebra_glm::{Vec3};
use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::material::Material;
use crate::plane::Plane;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::skybox::Skybox;

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32, eta_i: f32) -> Option<Vec3> {
    let cosi = incident.dot(normal).clamp(-1.0, 1.0);
    let eta = eta_i / eta_t;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        None
    } else {
        Some(eta * incident + (eta * cosi - k.sqrt()) * normal)
    }
}

fn is_in_frustum(camera: &Camera, cube: &Cube) -> bool {
    let min_bound = cube.center.coords - Vec3::new(cube.size / 2.0, cube.size / 2.0, cube.size / 2.0);
    let max_bound = cube.center.coords + Vec3::new(cube.size / 2.0, cube.size / 2.0, cube.size / 2.0);

    let frustum_min = camera.position.coords - Vec3::new(10.0, 10.0, 10.0);
    let frustum_max = camera.position.coords + Vec3::new(10.0, 10.0, 10.0);

    (min_bound.x >= frustum_min.x && max_bound.x <= frustum_max.x)
        && (min_bound.y >= frustum_min.y && max_bound.y <= frustum_max.y)
        && (min_bound.z >= frustum_min.z && max_bound.z <= frustum_max.z)
}

fn is_in_frustum_plane(_camera: &Camera, _plane: &Plane) -> bool {
    true
}



fn render_lod_cube(framebuffer: &mut Framebuffer, cube: &Cube, camera: &Camera, lights: &[Light]) {
    let distance = (camera.position.coords - cube.center.coords).magnitude();

    if distance > 20.0 {
        render_simple_cube(framebuffer, cube, camera, lights);
    } else {
        render_detailed_cube(framebuffer, cube, camera, lights);
    }
}

fn render_simple_cube(_framebuffer: &mut Framebuffer, _cube: &Cube, _camera: &Camera, _lights: &[Light]) {}

fn render_detailed_cube(_framebuffer: &mut Framebuffer, _cube: &Cube, _camera: &Camera, _lights: &[Light]) {}

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
    skybox.render_skybox(framebuffer);

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = std::f32::consts::PI / 3.0;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * fov.tan();
            let screen_y = screen_y * fov.tan();

            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.transform_direction(&ray_direction);

            let pixel_color = cast_ray(
                &camera.position.coords,
                &rotated_direction,
                objects,
                cubes,
                planes,
                suns,
                lights,
                0,
                skybox
            );

            framebuffer.set_pixel(x as usize, y as usize, pixel_color.to_hex());
        }
    }
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Sphere],
    cubes: &[Cube],
    planes: &[Plane],
    suns: &[Sphere],
    lights: &[Light],
    depth: u32,
    skybox: &Skybox,
) -> Color {
    if depth > 6 {
        return Color::new(0, 0, 0);
    }

    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects.iter().chain(suns.iter()) {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if intersect.distance < zbuffer {
                zbuffer = intersect.distance;
                closest_intersect = intersect;
            }
        }
    }

    for cube in cubes.iter() {
        if let Some(intersect) = cube.ray_intersect(ray_origin, ray_direction) {
            if intersect.distance < zbuffer {
                zbuffer = intersect.distance;
                closest_intersect = intersect;
            }
        }
    }

    for plane in planes.iter() {
        if let Some(intersect) = plane.ray_intersect(ray_origin, ray_direction) {
            if intersect.distance < zbuffer {
                zbuffer = intersect.distance;
                closest_intersect = intersect;
            }
        }
    }

    if !closest_intersect.is_intersecting {
        return skybox.get_color(ray_direction);
    }

    if closest_intersect.material.emissive != Color::new(0, 0, 0) {
        return closest_intersect.material.emissive;
    }

    let mut color_accumulator = Color::new(0, 0, 0);
    let ambient_light_intensity = 0.1;  // Ajusta este valor según lo que necesites
    let ambient_color = closest_intersect.material.color * ambient_light_intensity;
    for light in lights {
        let light_dir = (light.position - closest_intersect.point).normalize();
        let shadow_origin = closest_intersect.point + light_dir * 1e-4;

        let diffuse_intensity = closest_intersect.normal.dot(&light_dir).max(0.0) * light.intensity;
        let diffuse = closest_intersect.material.albedo[0] * diffuse_intensity;


        let mut in_shadow = false;

        for object in objects.iter().chain(suns.iter()) {
            if let Some(shadow_intersect) = object.ray_intersect(&shadow_origin, &light_dir) {
                if shadow_intersect.distance < (light.position - closest_intersect.point).magnitude() {
                    in_shadow = true;
                    break;
                }
            }
        }

        for cube in cubes.iter() {
            if let Some(shadow_intersect) = cube.ray_intersect(&shadow_origin, &light_dir) {
                if shadow_intersect.distance < (light.position - closest_intersect.point).magnitude() {
                    in_shadow = true;
                    break;
                }
            }
        }

        for plane in planes.iter() {
            if let Some(shadow_intersect) = plane.ray_intersect(&shadow_origin, &light_dir) {
                if shadow_intersect.distance < (light.position - closest_intersect.point).magnitude() {
                    in_shadow = true;
                    break;
                }
            }
        }

        let light_intensity = if in_shadow { 0.1 } else { 1.0 };
        let adjusted_diffuse = closest_intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        color_accumulator = color_accumulator + closest_intersect.material.color * adjusted_diffuse;
    }

    let reflect_dir = ray_direction - 2.0 * ray_direction.dot(&closest_intersect.normal) * closest_intersect.normal;
    let reflect_origin = closest_intersect.point + reflect_dir * 1e-4;
    let reflect_color = cast_ray(
        &reflect_origin,
        &reflect_dir.normalize(),
        objects,
        cubes,
        planes,
        suns,
        lights,
        depth + 1,
        skybox,
    );

    let refractive_index = closest_intersect.material.refractive_index;
    let refract_color = if refractive_index > 1.0 {
        if let Some(refract_dir) = refract(ray_direction, &closest_intersect.normal, refractive_index, 1.0) {
            let refract_origin = closest_intersect.point + refract_dir * 1e-4;
            cast_ray(
                &refract_origin,
                &refract_dir.normalize(),
                objects,
                cubes,
                planes,
                suns,
                lights,
                depth + 1,
                skybox
            )
        }  else {
            skybox.get_color(ray_direction)
        }
    } else {
        skybox.get_color(ray_direction)
    };

    let reflectivity = closest_intersect.material.albedo[2];
    let transparency = closest_intersect.material.albedo[3];
    color_accumulator * (1.0 - reflectivity - transparency)
        + reflect_color * reflectivity
        + refract_color * transparency
}

