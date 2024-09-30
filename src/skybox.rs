use crate::color::Color;
use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub struct Skybox {
    pub is_day: bool,
}

impl Skybox {
    pub fn new() -> Self {
        Skybox {
            is_day: true, // Start with day mode
        }
    }

    // Toggle between day and night sky
    pub fn toggle_day_night(&mut self) {
        self.is_day = !self.is_day;
        if self.is_day {
            println!("Switched to day sky.");
        } else {
            println!("Switched to night sky.");
        }
    }

    // Get color based on ray direction
    pub fn get_color(&self, ray_direction: &Vec3) -> Color {
        if self.is_day {
            self.generate_day_color_from_direction(ray_direction)
        } else {
            self.generate_night_color_from_direction(ray_direction)
        }
    }

    // Generate day sky color based on direction
    fn generate_day_color_from_direction(&self, ray_direction: &Vec3) -> Color {
        let t = (ray_direction.y + 1.0) / 2.0; // Normalize from -1 to 1 to 0 to 1

        // Interpolate between deep sky blue and light sky blue
        let top_color = Color::new(70, 130, 180); // Deep sky blue
        let horizon_color = Color::new(135, 206, 250); // Light sky blue

        Color {
            r: (top_color.r as f32 * (1.0 - t) + horizon_color.r as f32 * t) as u8,
            g: (top_color.g as f32 * (1.0 - t) + horizon_color.g as f32 * t) as u8,
            b: (top_color.b as f32 * (1.0 - t) + horizon_color.b as f32 * t) as u8,
        }
    }

    // Generate night sky color based on direction
    fn generate_night_color_from_direction(&self, ray_direction: &Vec3) -> Color {
        let t = (ray_direction.y + 1.0) / 2.0; // Normalize from -1 to 1 to 0 to 1

        // Interpolate between dark blue and black
        // Línea 59
        let top_color = Color::new(170, 108, 102); // Azul oscuro

        // Línea 60
        let horizon_color = Color::new(136, 97, 124); // Blanco cerca del horizonte

        Color {
            r: (top_color.r as f32 * (1.0 - t) + horizon_color.r as f32 * t) as u8,
            g: (top_color.g as f32 * (1.0 - t) + horizon_color.g as f32 * t) as u8,
            b: (top_color.b as f32 * (1.0 - t) + horizon_color.b as f32 * t) as u8,
        }
    }
}
