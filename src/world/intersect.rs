use crate::world::Object;
use crate::world::ray::Ray;

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