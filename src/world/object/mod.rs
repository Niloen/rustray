use crate::vector::Vector3;
use crate::world::intersect::Intersecting;
use crate::world::ray::Ray;
pub use sphere::Sphere;
pub use cube::Cube;
use crate::world::surface::Surface;

mod sphere;
mod cube;

#[derive(Debug)]
pub struct HitResult<'a> {
    pub position: Vector3,
    pub normal: Vector3,
    pub surface: Surface<'a>,
}

pub trait Object<'a>: Send + Sync + Intersecting<'a>{

    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}

