use crate::algebra::{Bounded, Distance, Point3, Vector3};
use crate::algebra::Ray;
pub use sphere::Sphere;
pub use cube::Cube;
pub use plane::Plane;

mod sphere;
mod cube;
mod plane;

pub type TextureCoords = (Distance, Distance);

#[derive(Debug)]
pub struct HitResult {
    pub position: Point3,
    pub normal: Vector3,
    pub coords: TextureCoords
}

pub trait Geometry: Send + Sync + Bounded {
    
    fn distance(&self, ray: &Ray) -> Option<Distance>;

    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}
