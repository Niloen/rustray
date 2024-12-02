use crate::algebra::Ray;
use crate::algebra::{Bounded, Distance, Point3, UnitVector3};
pub use cube::Cube;
pub use plane::Plane;
pub use sphere::Sphere;

mod sphere;
mod cube;
mod plane;

pub type TextureCoords = (Distance, Distance);

#[derive(Debug)]
pub struct HitResult {
    pub position: Point3,
    pub normal: UnitVector3,
    pub coords: TextureCoords
}

pub trait Geometry: Send + Sync + Bounded {
    
    fn distance(&self, ray: &Ray) -> Option<Distance>;

    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}
