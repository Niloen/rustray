use crate::vector::Vector3;
use crate::world::ray::Ray;
#[derive(Debug)]
pub struct HitResult {
    pub distance: f64,
    pub normal: Vector3
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        self.hit(ray).map(|r| r.distance)
    }

    fn hit(&self, r: &Ray) -> Option<HitResult>;
}

