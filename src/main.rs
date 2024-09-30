// main.rs

mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod material;
mod plane;
mod ray_intersect;
mod render;
mod skybox;
mod sphere;
mod texture;

use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::material::Material;
use crate::plane::Plane;
use crate::render::render;
use crate::skybox::Skybox;
use crate::sphere::Sphere;
use crate::texture::Texture;
use minifb::{Key, KeyRepeat, MouseButton, Window, WindowOptions};
use nalgebra::Point3; // Para manejar puntos 3D correctamente
use nalgebra::Vector3 as Vec3;
use std::time::{Duration, Instant};

fn main() {
    let mut framebuffer = Framebuffer::new(640, 480); // Resolución
    let mut skybox = Skybox::new(); // Inicializa el skybox

    // Define el objetivo alrededor del cual orbitar
    let target = Point3::new(0.5, 0.5, 0.5); // Centro del objeto (objetivo fijo)

    // Inicializa la cámara con posición, objetivo y vector up
    let mut camera = Camera::new(
        Point3::new(0.0, 1.0, 5.0), // Posición de la cámara
        target,                     // El objetivo fijo (punto central)
        Vec3::new(0.0, 1.0, 0.0),   // Vector up
    );

    // Define materiales para los soles
    let day_sun_material = Material::yellow_sun();
    let night_sun1_material = Material::red_giant();
    let night_sun2_material = Material::yellow_sun(); // Puedes definir otro material si lo prefieres

    // Inicializa los soles con el material de día
    let mut suns = vec![
        Sphere {
            center: Vec3::new(1.0, 7.0, -6.0), // Posición del primer sol
            radius: 1.0,
            material: day_sun_material.clone(),
        },
        Sphere {
            center: Vec3::new(6.0, 5.0, -7.5), // Posición del segundo sol
            radius: 0.7,
            material: day_sun_material.clone(),
        },
    ];

    // Inicializa las luces asociadas a los soles
    let mut lights = vec![
        Light::new(
            Vec3::new(1.0, 7.0, -6.0),
            day_sun_material.emissive,
            2.0, // Intensidad para el día
        ),
        Light::new(
            Vec3::new(6.0, 5.0, -7.5),
            day_sun_material.emissive,
            1.5, // Intensidad para el día
        ),
    ];

    // Crea el plano del suelo
    let ground_plane = Plane::new(
        Vec3::new(0.0, 0.0, 0.0), // Posición en Y = 0
        Vec3::new(0.0, 1.0, 0.0), // Normal apuntando hacia arriba
        Material::sand(),         // Material de arena para el suelo
    );
    let planes = vec![ground_plane];

    // Crea cubos para la estructura
    let cubes = vec![
        // Arenisca para la cúpula
        Cube::new(Point3::new(0.0, 0.5, 0.0), 1.0, Material::sandstone()), // Bloque central
        Cube::new(Point3::new(1.0, 0.5, 0.0), 1.0, Material::sandstone()), // Bloque lateral (derecha)
        Cube::new(Point3::new(-1.0, 0.5, 0.0), 1.0, Material::sandstone()), // Bloque lateral (izquierda)
        Cube::new(Point3::new(0.0, 0.5, 1.0), 1.0, Material::sandstone()),  // Bloque trasero
        Cube::new(Point3::new(0.0, 0.5, -1.0), 1.0, Material::sandstone()), // Bloque frontal
        // Segunda capa de la cúpula, usando arcilla para detalles
        Cube::new(Point3::new(0.5, 1.0, 0.0), 1.0, Material::clay()), // Bloque superior lateral (derecha)
        Cube::new(Point3::new(-0.5, 1.0, 0.0), 1.0, Material::clay()), // Bloque superior lateral (izquierda)
        Cube::new(Point3::new(0.0, 1.0, 0.5), 1.0, Material::clay()),  // Bloque superior trasero
        Cube::new(Point3::new(0.0, 1.0, -0.5), 1.0, Material::clay()), // Bloque superior frontal
        // Detalles adicionales (cajas metálicas) y metal oxidado para desgaste
        Cube::new(Point3::new(2.5, 0.25, 0.0), 0.5, Material::metal()), // Caja metálica (derecha)
        Cube::new(Point3::new(-2.5, 0.25, 0.0), 0.5, Material::rusted_metal()), // Caja de metal oxidado (izquierda)
    ];

    // Crea la ventana
    let mut window = Window::new(
        "Raytracer - Tatooine",
        framebuffer.width,
        framebuffer.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    // Velocidades para diversas acciones de la cámara
    let zoom_speed = 0.5; // Sensibilidad de zoom
    let orbit_speed = 0.01; // Sensibilidad de órbita
    let movement_speed = 0.8; // Sensibilidad de movimiento

    // Variables para rastrear la posición del mouse y el tiempo entre frames
    let mut last_mouse_pos = None;
    let mut last_frame = Instant::now();
    // En main.rs


    // Bucle principal de renderizado
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let delta_time = now - last_frame;
        last_frame = now;
        let delta_time_seconds = delta_time.as_secs_f32();

        // Zoom usando la rueda del mouse
        if let Some(scroll) = window.get_scroll_wheel() {
            camera.zoom(scroll.1 * zoom_speed * delta_time_seconds); // Ajusta el radio de la cámara
        }

        // Órbita de la cámara arrastrando con el botón izquierdo del mouse
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                if let Some((last_x, last_y)) = last_mouse_pos {
                    let delta_x = x - last_x;
                    let delta_y = y - last_y;

                    // Órbita basada en el arrastre del mouse
                    camera.orbit(delta_x as f32 * orbit_speed, delta_y as f32 * orbit_speed);
                }
                last_mouse_pos = Some((x, y));
            }
        } else {
            last_mouse_pos = None;
        }

        // Mover la cámara con las teclas WASD sin mover el objetivo
        if window.is_key_down(Key::W) {
            camera.move_up_global(movement_speed);
        }
        if window.is_key_down(Key::S) {
            camera.move_up_global(-movement_speed);
        }

        // Mover izquierda/derecha (teclas A/D) - a lo largo del eje X global
        if window.is_key_down(Key::A) {
            camera.move_right_global(-movement_speed);
        }
        if window.is_key_down(Key::D) {
            camera.move_right_global(movement_speed);
        }

        // Manejar la entrada para alternar entre día y noche
        if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
            skybox.toggle_day_night();

            if skybox.is_day {
                // Configuraciones para el día

                // Actualizar materiales de los soles
                suns[0].material = day_sun_material.clone();
                suns[1].material = day_sun_material.clone();

                // Actualizar propiedades de las luces
                lights[0].color = day_sun_material.emissive;
                lights[0].intensity = 2.0; // Intensidad para el día

                lights[1].color = day_sun_material.emissive;
                lights[1].intensity = 1.5; // Intensidad para el día
            } else {
                // Configuraciones para la noche (atardecer)

                // Actualizar materiales de los soles
                suns[0].material = night_sun1_material.clone();
                suns[1].material = night_sun2_material.clone();

                // Actualizar propiedades de las luces
                lights[0].color = night_sun1_material.emissive;
                lights[0].intensity = 1.0; // Intensidad reducida para la noche

                lights[1].color = night_sun2_material.emissive;
                lights[1].intensity = 0.8; // Intensidad reducida para la noche
            }
        }

        // Limpiar el framebuffer antes de renderizar
        framebuffer.clear();

        // Renderizar la escena
        render(
            &mut framebuffer,
            &suns,
            &cubes,
            &planes,
            &camera,
            &suns,
            &lights,
            &skybox,
        );

        // Actualizar la ventana con el nuevo frame
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();

        // Controlar la tasa de frames a aproximadamente 60 FPS
        let frame_time = now.elapsed();
        let target_frame_duration = Duration::from_millis(16); // Aproximadamente 60 FPS
        if frame_time < target_frame_duration {
            std::thread::sleep(target_frame_duration - frame_time);
        }
    }
}
