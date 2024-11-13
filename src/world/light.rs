use std::ops::Neg;
use image::{Pixel, Rgb};
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
    
    pub fn illuminate(&self, position: Vector3, normal: Vector3) -> Rgb<f64> {
        let mut fraction = self.towards(position).direction.cos_angle(normal).neg();
        if fraction < 0.0 {
            fraction = 0.0
        }

        self.color.map(|c|c * fraction)
    }
}