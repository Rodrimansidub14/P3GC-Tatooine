use crate::color::Color;
use crate::texture::Texture;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,                   // Default color (used if no texture is applied)
    pub albedo: [f32; 4],               // [diffuse, specular, reflectivity, transparency]
    pub specular: f32,                  // Specular intensity for reflections
    pub refractive_index: f32,          // Refractive index for transparency/refraction
    pub emissive: Color,                // Color for emissive materials
    pub has_texture: bool,              // If true, the material uses a texture
    pub texture: Option<Arc<Texture>>,  // Optional texture for materials
    pub normal_map: Option<Arc<Texture>> // Optional normal map for materials
}

impl Material {
    pub fn black() -> Self {
        Material {
            color: Color::new(0, 0, 0),
            albedo: [0.0, 0.0, 0.0, 0.0],
            specular: 0.0,
            refractive_index: 1.0,
            emissive: Color::new(0, 0, 0),
            has_texture: false,
            texture: None,
            normal_map: None,
        }
    }

    pub fn new_with_texture(
        color: Color,
        albedo: [f32; 4],
        specular: f32,
        refractive_index: f32,
        texture: Option<Arc<Texture>>,
        normal_map: Option<Arc<Texture>>, // Added normal map as a parameter
    ) -> Self {
        Material {
            color,
            albedo,
            specular,
            refractive_index,
            emissive: Color::new(0, 0, 0),
            has_texture: texture.is_some(),
            texture,
            normal_map,
        }
    }

    pub fn new(
        color: Color,
        albedo: [f32; 4],
        specular: f32,
        refractive_index: f32,
    ) -> Self {
        Material {
            color,
            albedo,
            specular,
            refractive_index,
            emissive: Color::new(0, 0, 0),
            has_texture: false,
            texture: None,
            normal_map: None, // Set as None for materials without a normal map
        }
    }

    // Function to get the diffuse color based on texture coordinates (u, v)
    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if self.has_texture {
            if let Some(texture) = &self.texture {
                let x = (u * (texture.width as f32 - 1.0)) as usize;
                let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
                return texture.get_color(x, y);
            }
        }
        self.color
    }

    pub fn yellow_sun() -> Self {
        Material {
            color: Color::new(255, 255, 102), // Yellow for the sun
            albedo: [1.0, 0.0, 0.0, 0.0],
            specular: 250.0,
            refractive_index: 1.0,
            emissive: Color::new(255, 255, 102),
            has_texture: false,
            texture: None,
            normal_map: None, // Suns typically don't need normal maps
        }
    }

    pub fn red_giant() -> Self {
        Material {
            color: Color::new(255, 69, 0),    // Red-orange for the giant sun
            albedo: [1.0, 0.0, 0.0, 0.0],
            specular: 200.0,
            refractive_index: 1.0,
            emissive: Color::new(255, 69, 0),
            has_texture: false,
            texture: None,
            normal_map: None,
        }
    }
}
