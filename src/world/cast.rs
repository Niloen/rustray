use crate::world::object::HitResult;
use crate::world::ray::Ray;
pub trait RayCaster: Send + Sync {
    fn cast(&self, ray: &Ray, depth: u32) -> Option<HitResult>;
}