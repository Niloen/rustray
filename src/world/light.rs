use image::{Pixel, Rgb};
use crate::vector::{Point3, Vector3, VectorOps};
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
        let mut fraction = self.towards(position).direction.cos_angle(&normal);
        if fraction < 0.0 {
            fraction = 0.0
        }

        self.color.map(|c|c * fraction)
    }
}