use crate::scene::object::Object;
use crate::scene::geometry::Geometry;
use crate::algebra::Ray;

#[non_exhaustive]
pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Object
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, object: &'a Object) -> Self {
        Self {
            distance,
            object
        }
    }
}

pub trait Intersecting: Send + Sync {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}

impl Intersecting for Object {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.distance(ray).map(|distance| Intersection::new(
            distance,
            self
        ))
    }
}
