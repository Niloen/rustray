use image::Rgb;
use crate::vector::Vector3;
use crate::world::material::Material;
use crate::world::ray::Ray;
#[derive(Debug)]
pub struct HitResult<'a> {
    pub distance: f64,
    pub position: Vector3,
    pub normal: Vector3,
    pub color: Rgb<f64>,
    pub material: &'a dyn Material,
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

pub trait Intersecting<'a>: Send + Sync {
    fn intersects<'b, 'z>(&'b self, ray: &Ray) -> Option<Intersection<'z, 'a>>
    where
        'a: 'z,
        'b: 'z;
}

pub trait Object<'a>: Send + Sync + Intersecting<'a>{

    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}

