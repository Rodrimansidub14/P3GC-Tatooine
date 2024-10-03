// src/skybox.rs

use nalgebra_glm::Vec3;
use crate::color::Color;

pub struct Skybox {
    pub is_day: bool,
}

impl Skybox {
    pub fn new() -> Self {
        Skybox {
            is_day: true, // Comienza en modo día
        }
    }

    // Alterna entre día y noche
    pub fn toggle_day_night(&mut self) {
        self.is_day = !self.is_day;
        if self.is_day {
            println!("Cambiado a cielo de día.");
        } else {
            println!("Cambiado a cielo de atardecer/noche.");
        }
    }

    // Genera el color del cielo basado en la dirección del rayo
    pub fn get_color(&self, ray_direction: &Vec3) -> Color {
        if self.is_day {
            self.generate_day_color_from_direction(ray_direction)
        } else {
            self.generate_sunset_color_from_direction(ray_direction)
        }
    }

    // Genera el color para el cielo de día
    fn generate_day_color_from_direction(&self, ray_direction: &Vec3) -> Color {
        let t = (ray_direction.y + 1.0) / 2.0;
    
        // Define the four colors for the gradient
        let top_color = Color::new(135, 206, 235);        // Sky blue (top of the sky)
        let middle_top_color = Color::new(176, 224, 230); // Lighter blue
        let middle_bottom_color = Color::new(250, 235, 215); // Very light, approaching white
        let horizon_color = Color::new(255, 255, 255);    // White (at the horizon)
    
        let color = if t > 0.66 {
            // Interpolate between top_color and middle_top_color
            let factor = (t - 0.66) * 3.0;
            Color {
                r: (top_color.r as f32 * factor + middle_top_color.r as f32 * (1.0 - factor)) as u8,
                g: (top_color.g as f32 * factor + middle_top_color.g as f32 * (1.0 - factor)) as u8,
                b: (top_color.b as f32 * factor + middle_top_color.b as f32 * (1.0 - factor)) as u8,
            }
        } else if t > 0.33 {
            // Interpolate between middle_top_color and middle_bottom_color
            let factor = (t - 0.33) * 3.0;
            Color {
                r: (middle_top_color.r as f32 * factor + middle_bottom_color.r as f32 * (1.0 - factor)) as u8,
                g: (middle_top_color.g as f32 * factor + middle_bottom_color.g as f32 * (1.0 - factor)) as u8,
                b: (middle_top_color.b as f32 * factor + middle_bottom_color.b as f32 * (1.0 - factor)) as u8,
            }
        } else {
            // Interpolate between middle_bottom_color and horizon_color
            let factor = t * 3.0;
            Color {
                r: (middle_bottom_color.r as f32 * factor + horizon_color.r as f32 * (1.0 - factor)) as u8,
                g: (middle_bottom_color.g as f32 * factor + horizon_color.g as f32 * (1.0 - factor)) as u8,
                b: (middle_bottom_color.b as f32 * factor + horizon_color.b as f32 * (1.0 - factor)) as u8,
            }
        };
    
        color
    }
    

    // Genera el color para el cielo de atardecer/noche
    fn generate_sunset_color_from_direction(&self, ray_direction: &Vec3) -> Color {
        let t = (ray_direction.y + 1.0) / 2.0;
    
        // Define the four colors for sunset gradient
        let top_color = Color::new(138, 68, 94);        // Red-orange (top of the sky)
        let middle_top_color = Color::new(126,67,95);  // Orange
        let middle_bottom_color = Color::new(248, 90, 62); // Light peach
        let horizon_color = Color::new(255,119,51);    // Yellow (at the horizon)
    
        let color = if t > 0.66 {
            // Interpolate between top_color and middle_top_color
            let factor = (t - 0.66) * 3.0;
            Color {
                r: (top_color.r as f32 * factor + middle_top_color.r as f32 * (1.0 - factor)) as u8,
                g: (top_color.g as f32 * factor + middle_top_color.g as f32 * (1.0 - factor)) as u8,
                b: (top_color.b as f32 * factor + middle_top_color.b as f32 * (1.0 - factor)) as u8,
            }
        } else if t > 0.33 {
            // Interpolate between middle_top_color and middle_bottom_color
            let factor = (t - 0.33) * 3.0;
            Color {
                r: (middle_top_color.r as f32 * factor + middle_bottom_color.r as f32 * (1.0 - factor)) as u8,
                g: (middle_top_color.g as f32 * factor + middle_bottom_color.g as f32 * (1.0 - factor)) as u8,
                b: (middle_top_color.b as f32 * factor + middle_bottom_color.b as f32 * (1.0 - factor)) as u8,
            }
        } else {
            // Interpolate between middle_bottom_color and horizon_color
            let factor = t * 3.0;
            Color {
                r: (middle_bottom_color.r as f32 * factor + horizon_color.r as f32 * (1.0 - factor)) as u8,
                g: (middle_bottom_color.g as f32 * factor + horizon_color.g as f32 * (1.0 - factor)) as u8,
                b: (middle_bottom_color.b as f32 * factor + horizon_color.b as f32 * (1.0 - factor)) as u8,
            }
        };
    
        color
    }
    

    // Renderiza el skybox en el framebuffer
    pub fn render_skybox(&self, framebuffer: &mut crate::framebuffer::Framebuffer) {
        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                // Coordenadas normalizadas
                let screen_x = (2.0 * x as f32) / framebuffer.width as f32 - 1.0;
                let screen_y = -(2.0 * y as f32) / framebuffer.height as f32 + 1.0;

                // Asumiendo una cámara con FOV de 90 grados
                let fov = std::f32::consts::FRAC_PI_2;
                let aspect_ratio = framebuffer.width as f32 / framebuffer.height as f32;
                let screen_x = screen_x * aspect_ratio * fov.tan();
                let screen_y = screen_y * fov.tan();

                let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

                let color = self.get_color(&ray_direction);
                framebuffer.set_pixel(x as usize, y as usize, color.to_hex());
            }
        }
    }
}
