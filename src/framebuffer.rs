// src/framebuffer.rs

use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>, // Almacenamiento de colores en formato hexadecimal
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    // Método para limpiar el framebuffer (establecer todos los píxeles a negro)
    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = 0;
        }
    }

    // Método para establecer el color de un píxel específico
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    // Método para dibujar un punto en el framebuffer
    pub fn point(&mut self, x: usize, y: usize) {
        // Este método ya no es necesario si usamos set_pixel directamente
        // Puedes eliminarlo si no se usa en otros lugares
    }
}
