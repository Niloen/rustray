use crate::scene::geometry::HitResult;
use crate::algebra::Ray;
use crate::scene::ray::RayCaster;
use image::{Pixel, Rgb};
use std::fmt::Debug;

pub trait Material: Send + Sync + Debug {
    /// Calculates the color for the material at an intersection point.
    /// Takes the ray that hit the object, the hit result, the ray caster function, and the recursion depth.
    fn shade(&self, ray: &Ray, hit: &HitResult, color: Rgb<f64>, caster: &dyn RayCaster, depth: u32) -> Rgb<f64>;
    fn clone_box(&self) -> Box<dyn Material>;
}

#[derive(Debug, Clone)]
pub struct BaseMaterial {
    pub reflectivity: f64,   // 0 for diffuse, higher values for reflective
    pub emission: Rgb<f64>,  // Non-zero values make the material emissive
    pub refractive: f64
}

impl Material for BaseMaterial {
    fn shade(&self, ray: &Ray, hit: &HitResult, color: Rgb<f64>, caster: &dyn RayCaster, depth: u32) -> Rgb<f64> {
        // Basic shading logic with adjustable parameters

        let mut color = if self.reflectivity < 1.0 {
            if color == BaseMaterial::BLACK {
                color
            } else {
                caster.direct_lightning(&Ray::new(hit.position, hit.normal))
                    .map(|c| c * (1.0 - self.reflectivity))
                    .map2(&color, |c1, c2|c1 * c2)
            }
        } else {
            Rgb([0.0, 0.0, 0.0]) // Skip diffuse lighting for fully reflective surfaces
        };
        
        // Reflection
        if self.reflectivity > 0.0 && depth > 0 {
            let reflected_color = Self::reflected_color(ray, hit, caster, depth);

            color = color.map2(&reflected_color, |c1, c2|
                c1 * (1.0 - self.reflectivity) + c2 * self.reflectivity);
        }

        if self.refractive > 1.0 && depth > 0 {
            let refracted_color = Self::refracted_color(ray, hit, caster, depth, self.refractive);
            color = color.map2(&refracted_color, |c1, c2| c1 + c2 * (1.0 - self.reflectivity));
        }

        // Emission
        color = color.map2(&self.emission, |c1, c2| c1 + c2);

        color
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl BaseMaterial {
    pub const DEFAULT: BaseMaterial = BaseMaterial {
        emission: Rgb([0.0, 0.0, 0.0]),
        reflectivity: 0.0,
        refractive: 1.0
    };

    const BLACK: Rgb<f64> = Rgb([0.0, 0.0, 0.0]);

    fn reflected_color(ray: &Ray, hit: &HitResult, caster: &dyn RayCaster, depth: u32) -> Rgb<f64> {
        let reflected_direction = ray.reflect(hit.normal).direction;
        // Adjust along normal to avoid self-intersection
        let reflected_ray = Ray::new(hit.position + hit.normal * 0.000001, reflected_direction);
        let reflected_color = caster.cast(&reflected_ray, depth - 1);
        reflected_color
    }

    fn refracted_color(ray: &Ray, hit: &HitResult, caster: &dyn RayCaster, depth: u32, refractive_index: f64) -> Rgb<f64> {
        let n1 = 1.0; // Assuming ray originates in air with refractive index 1.0
        let n2 = refractive_index;

        let cos_i = -hit.normal.dot(&ray.direction).max(-1.0).min(1.0);
        let (n1, n2, normal) = if cos_i < 0.0 {
            // Inside the material; flip normal
            (n2, n1, -hit.normal)
        } else {
            (n1, n2, hit.normal)
        };

        let eta = n1 / n2;
        let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);

        if sin_t2 > 1.0 {
            // Total internal reflection
            return Rgb([0.0, 0.0, 0.0]);
        }

        let cos_t = (1.0 - sin_t2).sqrt();
        let refracted_direction = ray.direction * eta + normal * (eta * cos_i - cos_t);
        let refracted_ray = Ray::new(hit.position - hit.normal * 0.000001, refracted_direction);
        caster.cast(&refracted_ray, depth - 1)
    }
}
