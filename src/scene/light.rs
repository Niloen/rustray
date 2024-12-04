use crate::algebra::Ray;
use crate::algebra::{Point3, UnitVector3, Vector3, VectorOps};
use crate::scene::{Color, ColorPart};
use image::{Pixel, Rgb};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub ray: Ray,
    pub color: Color,
}

impl Light {
    const BLACK: Color = Rgb([0.0, 0.0, 0.0]);
    
    pub fn new(ray: Ray, color: Color) -> Self {
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
    
    pub fn illuminate(&self, position: Point3, normal: UnitVector3) -> Color {
        let direction_to_light = self.towards_direction(position);
        let fraction = direction_to_light.cos_angle(&normal) as ColorPart;
        
        if fraction <= 0.0 {
            return Self::BLACK;
        }
        
        self.color.map(|c|c * fraction)
    }
}