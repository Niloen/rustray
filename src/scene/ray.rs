use crate::algebra::{Point3, Ray, UnitVector3};
use crate::scene::Color;

pub trait RayCaster: Sync {
    fn cast(&self, ray: &Ray, depth: u32) -> Color;

    fn direct_lightning(&self, position: &Point3, normal: &UnitVector3) -> Color;
}