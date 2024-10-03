// src/color.rs

use std::ops::{Add, Mul};
use nalgebra::Vector3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
    // Método para convertir el color a un valor hexadecimal (RGB)
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    pub fn from_vec(normal: Vector3<f32>) -> Self {
        // Convertimos de [-1, 1] a [0, 255] para cada componente
        let r = ((normal.x + 1.0) * 0.5 * 255.0) as u8;
        let g = ((normal.y + 1.0) * 0.5 * 255.0) as u8;
        let b = ((normal.z + 1.0) * 0.5 * 255.0) as u8;
        Color::new(r, g, b)
    }
}

// Implementación del trait Add para Color
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: (self.r as u16 + other.r as u16).min(255) as u8,
            g: (self.g as u16 + other.g as u16).min(255) as u8,
            b: (self.b as u16 + other.b as u16).min(255) as u8,
        }
    }
}

// Implementación del trait Mul<f32> para Color
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: ((self.r as f32) * scalar).min(255.0) as u8,
            g: ((self.g as f32) * scalar).min(255.0) as u8,
            b: ((self.b as f32) * scalar).min(255.0) as u8,
        }
    }
}
