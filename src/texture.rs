// texture.rs

use image::{DynamicImage, GenericImageView};
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Texture {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
}



impl Texture {
    pub fn from_file(path: &str) -> Self {
        let image = image::open(path).unwrap_or_else(|e| {
            panic!("Failed to load texture image from {}: {}", path, e);
        });
        let (width, height) = image.dimensions();

        Texture { image, width, height }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        // Aseguramos que u y v estén en el rango [0, 1]
        let u = u.fract();
        let v = v.fract();

        // Convertimos u y v a coordenadas de píxel
        let x = (u * self.width as f32) as u32 % self.width;
        let y = ((1.0 - v) * self.height as f32) as u32 % self.height;

        // Obtenemos el píxel de la textura
        let pixel = self.image.get_pixel(x, y).0; // Access the inner data directly

        // Convertimos el píxel a Color
        Color::new(pixel[0] as u32, pixel[1] as u32, pixel[2] as u32)
    }
}
