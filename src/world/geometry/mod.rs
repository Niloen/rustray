use crate::vector::{Point3, Vector3};
use crate::world::ray::Ray;
pub use sphere::Sphere;
pub use cube::Cube;
pub use plane::Plane;
mod sphere;
mod cube;
mod plane;

pub type TextureCoords = (f64, f64);

#[derive(Debug)]
pub struct HitResult {
    pub position: Point3,
    pub normal: Vector3,
    pub coords: TextureCoords
}

pub trait Geometry: Send + Sync {
    
    fn distance(&self, ray: &Ray) -> Option<f64>;

    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}
