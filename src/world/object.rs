use image::Rgb;
use crate::vector::Vector3;
use crate::world::ray::Ray;
#[derive(Debug, Clone)]
pub struct HitResult {
    pub distance: f64,
    pub normal: Vector3,
    pub color: Rgb<f64>
}

pub struct Intersection {
    pub distance: f64,
    result: Box<dyn Fn () -> HitResult>,
}

impl Intersection {
    /// Creates a new `Intersection` from a `HitResult` with lazy evaluation.
    pub fn from_result(hr: HitResult) -> Self {
        Self {
            distance: hr.distance,
            result: Box::new(move || hr.clone()), // Clones `hr` on each call
        }
    }

    /// Retrieves the `HitResult`, triggering lazy evaluation if necessary.
    pub fn get_result(&self) -> HitResult {
        (self.result)()
    }
}

pub trait Object: Send + Sync {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}
