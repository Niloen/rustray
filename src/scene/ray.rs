use crate::algebra::Ray;
use crate::scene::Color;

pub trait RayCaster: Sync {
    fn cast(&self, ray: &Ray, depth: u32) -> Color;

    fn direct_lightning(&self, normal_ray: &Ray) -> Color;
}