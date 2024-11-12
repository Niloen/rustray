use image::Rgb;
use crate::vector::Vector3;
use crate::world::ray::Ray;

pub struct Light {
    pub ray: Ray,
    pub color: Rgb<f64>,
}

impl Light {
    pub fn new(ray: Ray, color: Rgb<f64>) -> Self {
        Light {
            ray,
            color
        }
    }
    
    pub fn towards(&self, position: Vector3) -> Ray {
        return Ray::new(position, position - self.ray.origin)
    }
    
    pub fn distance_to(&self, position: Vector3) -> f64 {
        (position - self.ray.origin).length()
    }
}