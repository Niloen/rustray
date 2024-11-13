use image::Rgb;
use crate::world::ray::Ray;
pub trait RayCaster: Send + Sync {
    fn cast(&self, ray: &Ray, depth: u32) -> Rgb<f64>;
}