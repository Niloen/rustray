use image::Rgb;
use crate::world::ray::Ray;
pub trait RayCaster: Send + Sync {
    fn cast(&self, ray: &Ray, depth: u32) -> Rgb<f64>;
    
    fn direct_lightning(&self, normal_ray: &Ray) -> Rgb<f64>;
}

