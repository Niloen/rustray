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
        Ray::new(position, self.towards_direction(position))
    }
    
    pub fn towards_direction(&self, position: Vector3) -> Vector3 {
        self.ray.origin - position
    }
    
    pub fn distance_to(&self, position: Vector3) -> f64 {
        return self.towards_direction(position).length()
    }
    
    pub fn illuminate(&self, position: Vector3, normal: Vector3) -> Rgb<f64> {
        let mut fraction = self.towards(position).direction.cos_angle(&normal);
        if fraction < 0.0 {
            fraction = 0.0
        }

        self.color.map(|c|c * fraction)
    }
}