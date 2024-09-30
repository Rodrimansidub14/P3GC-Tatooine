// material.rs

use crate::color::Color;
use std::sync::Arc;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub albedo: [f32; 4], // [difuso, especular, reflectividad, transparencia]
    pub specular: f32,
    pub refractive_index: f32,
    pub emissive: Color,
}



impl Material {
    pub fn black() -> Self {
        Material {
            color: Color::new(0, 0, 0), // Color negro
            albedo: [0.0, 0.0, 0.0, 0.0],
            specular: 0.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn sand() -> Self {
        Material {
            color: Color::new(194, 178, 128),
            albedo: [0.9, 0.1, 0.0, 0.0], // Reflectividad y transparencia en 0.0
            specular: 10.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn metal() -> Self {
        Material {
            color: Color::new(192, 192, 192),
            albedo: [0.6, 0.3, 0.1, 0.0], // Reflectividad reducida a 0.1
            specular: 250.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn sandstone() -> Self {
        Material {
            color: Color::new(205, 170, 125),
            albedo: [0.9, 0.2, 0.0, 0.0], // Reflectividad y transparencia en 0.0
            specular: 50.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn clay() -> Self {
        Material {
            color: Color::new(160, 82, 45),
            albedo: [0.7, 0.3, 0.0, 0.0], // Reflectividad y transparencia en 0.0
            specular: 15.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn wood() -> Self {
        Material {
            color: Color::new(139, 69, 19), // Color de madera
            albedo: [0.8, 0.2, 0.0, 0.0], // Reflectividad y transparencia en 0.0
            specular: 50.0,
            refractive_index: 1.0,
            emissive: Color::black(),
        }
    }

    pub fn rusted_metal() -> Self {
        Material {
            color: Color::new(139, 69, 19),
            albedo: [0.6, 0.3, 0.1, 0.0], // Reflectividad reducida a 0.1
            specular: 100.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn glass() -> Self {
        Material {
            color: Color::new(200, 200, 255),
            albedo: [0.0, 0.5, 0.1, 0.8], // Transparencia alta
            specular: 250.0,
            refractive_index: 1.5,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn concrete() -> Self {
        Material {
            color: Color::new(130, 130, 130),
            albedo: [0.8, 0.2, 0.0, 0.0], // Reflectividad y transparencia en 0.0
            specular: 10.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
        }
    }

    pub fn yellow_sun() -> Self {
        Material {
            color: Color::new(255, 255, 102),
            albedo: [0.9, 0.1, 0.0, 0.0], // Reflectividad y transparencia en 0.0
            specular: 250.0,
            refractive_index: 1.0,
            emissive: Color::new(255, 255, 102),
        }
    }

    pub fn red_giant() -> Self {
        Material {
            color: Color::new(255, 69, 0),
            albedo: [0.8, 0.2, 0.0, 0.0], // Reflectividad y transparencia en 0.0
            specular: 200.0,
            refractive_index: 1.0,
            emissive: Color::new(255, 69, 0),
        }
    }
}
