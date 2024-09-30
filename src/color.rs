// color.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Color {
            r: r.min(255) as u8,
            g: g.min(255) as u8,
            b: b.min(255) as u8,
        }
    }

    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn clamp(&self) -> Self {
        Self {
            r: self.r.min(255).max(0),
            g: self.g.min(255).max(0),
            b: self.b.min(255).max(0),
        }
    }

    pub fn lerp(start: Color, end: Color, t: f32) -> Color {
        let r = (start.r as f32 + t * (end.r as f32 - start.r as f32)).clamp(0.0, 255.0) as u8;
        let g = (start.g as f32 + t * (end.g as f32 - start.g as f32)).clamp(0.0, 255.0) as u8;
        let b = (start.b as f32 + t * (end.b as f32 - start.b as f32)).clamp(0.0, 255.0) as u8;
        Color { r, g, b }
    }
}

// Implement arithmetic operations for Color
use std::ops::{Add, Mul, Div};

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Color {
            r: ((self.r as f32 * factor).clamp(0.0, 255.0)) as u8,
            g: ((self.g as f32 * factor).clamp(0.0, 255.0)) as u8,
            b: ((self.b as f32 * factor).clamp(0.0, 255.0)) as u8,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Color {
            r: ((self.r as u32 * other.r as u32) / 255).min(255) as u8,
            g: ((self.g as u32 * other.g as u32) / 255).min(255) as u8,
            b: ((self.b as u32 * other.b as u32) / 255).min(255) as u8,
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, divisor: f32) -> Self {
        Color {
            r: ((self.r as f32 / divisor).clamp(0.0, 255.0)) as u8,
            g: ((self.g as f32 / divisor).clamp(0.0, 255.0)) as u8,
            b: ((self.b as f32 / divisor).clamp(0.0, 255.0)) as u8,
        }
    }
}
