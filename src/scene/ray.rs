use image::Rgb;
use crate::algebra::Ray;

pub trait RayCaster: Sync {
    fn cast(&self, ray: &Ray, depth: u32) -> Rgb<f64>;

    fn direct_lightning(&self, normal_ray: &Ray) -> Rgb<f64>;
}