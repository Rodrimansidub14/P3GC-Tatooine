extern crate image;
use image::{ImageReader, Pixel, DynamicImage, GenericImageView};
use std::fmt;
use std::sync::Arc;
use crate::color::Color;
use nalgebra_glm::{Vec3};

#[derive(Clone, PartialEq)] // Added PartialEq for texture comparison
pub struct Texture {
    pub image: DynamicImage,
    pub width: usize,
    pub height: usize,
    pub color_array: Vec<Color>,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut texture = Texture {
            image: img,
            width,
            height,
            color_array: vec![Color::black(); width * height],
        };
        texture.load_color_array();
        texture
    }

    fn load_color_array(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.image.get_pixel(x as u32, y as u32).to_rgb();
                let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                self.color_array[y * self.width + x] = Color::new(pixel[0], pixel[1], pixel[2]); // Updated to Color::new
            }
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            Color::new(255, 0, 255) // Default magenta color in case of out of bounds
        } else {
            self.color_array[y * self.width + x] // Corrected indexing
        }
    }
    pub fn new_normal_map(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut texture = Texture {
            image: img,
            width,
            height,
            color_array: vec![Color::black(); width * height],  // No necesitas color
        };
        texture.load_normal_map();  // Carga la textura del normal map
        texture
    }

    // Método que carga un normal map y convierte los valores RGB a un vector normalizado
    fn load_normal_map(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.image.get_pixel(x as u32, y as u32).to_rgb();
                let r = (pixel[0] as f32 / 255.0) * 2.0 - 1.0; // Convertir de 0-255 a -1.0 a 1.0
                let g = (pixel[1] as f32 / 255.0) * 2.0 - 1.0;
                let b = (pixel[2] as f32 / 255.0) * 2.0 - 1.0;

                // Convertir los valores a un vector 3D y normalizarlo
                let normal = Vec3::new(r, g, b).normalize();
                self.color_array[y * self.width + x] = Color::from_vec(normal);  // Puedes adaptar `Color` a tu caso
            }
        }
    }

    // Obtener el valor del normal map en las coordenadas `x, y`
    pub fn get_normal(&self, x: usize, y: usize) -> Vec3 {
        if x >= self.width || y >= self.height {
            Vec3::new(0.0, 0.0, 1.0)  // Vector normal por defecto (hacia arriba)
        } else {
            let color = self.color_array[y * self.width + x];
            Vec3::new(color.r as f32, color.g as f32, color.b as f32)
        }
    }
}


impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Texture")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
