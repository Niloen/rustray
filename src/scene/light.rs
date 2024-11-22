use image::{Pixel, Rgb};
use crate::algebra::{Point3, Vector3, VectorOps};
use crate::algebra::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub ray: Ray,
    pub color: Rgb<f64>,
}

impl Light {
    const BLACK: Rgb<f64> = Rgb([0.0, 0.0, 0.0]);
    
    pub fn new(ray: Ray, color: Rgb<f64>) -> Self {
        Light {
            ray,
            color
        }
    }
    
    pub fn towards(&self, position: Point3) -> Ray {
        Ray::new(position, self.towards_direction(position))
    }
    
    pub fn towards_direction(&self, position: Point3) -> Vector3 {
        self.ray.origin - position
    }
    
    pub fn distance_to(&self, position: Point3) -> f64 {
        return self.towards_direction(position).magnitude()
    }
    
    pub fn illuminate(&self, position: Point3, normal: Vector3) -> Rgb<f64> {
        let direction_to_light = self.towards_direction(position);
        let fraction = direction_to_light.cos_angle(&normal);
        
        if fraction <= 0.0 {
            return Self::BLACK;
        }
        
        self.color.map(|c|c * fraction)
    }
}