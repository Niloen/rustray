use std::fmt::Debug;
use image::{Pixel, Rgb};
use crate::world::object::HitResult;
use crate::world::ray::Ray;
use crate::world::RayCaster;

pub trait Material: Send + Sync + Debug {
    /// Calculates the color for the material at an intersection point.
    /// Takes the ray that hit the object, the hit result, the ray caster function, and the recursion depth.
    fn shade(&self, ray: &Ray, hit: &HitResult, caster: &dyn RayCaster, depth: u32) -> Rgb<f64>;
}

#[derive(Debug)]
pub struct BaseMaterial {
    pub reflectivity: f64,   // 0 for diffuse, higher values for reflective
    pub transparency: f64,   // 0 for opaque, 1 for fully transparent
    pub emission: Rgb<f64>,  // Non-zero values make the material emissive
}

impl Material for BaseMaterial {
    fn shade(&self, ray: &Ray, hit: &HitResult, caster: &dyn RayCaster, depth: u32) -> Rgb<f64> {
        // Basic shading logic with adjustable parameters
        let mut color = hit.color;

        // Reflection
        if self.reflectivity > 0.0 && depth > 0 {
            let reflected_color = Self::reflected_color(ray, hit, caster, depth);

            color = color.map2(&reflected_color, |c1, c2|
                c1 * (1.0 - self.reflectivity) + c2 * self.reflectivity);
        }

        // Emission
        color = color.map2(&self.emission, |c1, c2| c1 + c2);

        color
    }
}

impl BaseMaterial {
    pub const DEFAULT: BaseMaterial = BaseMaterial {
        transparency: 0.0,
        emission: Rgb([0.0, 0.0, 0.0]),
        reflectivity: 0.0
    };
    
    fn reflected_color(ray: &Ray, hit: &HitResult, caster: &dyn RayCaster, depth: u32) -> Rgb<f64> {
        let reflected_direction = ray.reflect(hit.normal).direction;
        let reflected_ray = Ray::new(hit.position, reflected_direction);
        let reflected_color = caster.cast(&reflected_ray, depth - 1);
        reflected_color
    }
}
