use image::Rgb;
use crate::vector::Vector3;
use crate::world::ray::Ray;
#[derive(Debug, Clone)]
pub struct HitResult {
    pub distance: f64,
    pub normal: Vector3,
    pub color: Rgb<f64>,
}

pub struct Intersection<'a, 'b> {
    pub distance: f64,
    pub object: &'a dyn Object<'b>
}

impl<'a, 'b> Intersection<'a, 'b> {
    pub fn new(distance: f64, object: &'a dyn Object<'b>) -> Self {
        Self {
            distance,
            object
        }
    }
}

pub trait Object<'a>: Send + Sync {
    fn intersects<'b, 'z>(&'b self, ray: &Ray) -> Option<Intersection<'z, 'a>>
    where
        'a: 'z,
        'b: 'z;

    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}
