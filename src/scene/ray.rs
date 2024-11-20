use image::Rgb;
use crate::algebra::{Point3, Vector3};

#[derive(Debug)]
#[non_exhaustive]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
    pub fn at(&self, distance: f64) -> Point3 {
        self.origin + self.direction * distance
    }

    /// Calculates the reflected ray given a normal vector at the intersection point.
    pub fn reflect(&self, normal: Vector3) -> Ray {
        let reflected_direction = self.direction - normal * 2.0 * self.direction.dot(&normal);
        Ray::new(self.origin, reflected_direction)
    }    
}

pub trait RayCaster: Send + Sync {
    fn cast(&self, ray: &Ray, depth: u32) -> Rgb<f64>;
    
    fn direct_lightning(&self, normal_ray: &Ray) -> Rgb<f64>;
}