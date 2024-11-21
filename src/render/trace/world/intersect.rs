use std::sync::Arc;
use crate::scene::object::Object;
use crate::scene::geometry::Geometry;
use crate::algebra::{Bounded, BoundingBox, Ray};

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

pub trait Intersecting: Send + Sync + Bounded {
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

impl Intersecting for Arc<dyn Intersecting> {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.as_ref().intersects(ray)
    }
}

impl Bounded for Arc<dyn Intersecting> {

    fn bounding_box(&self) -> BoundingBox {
        self.as_ref().bounding_box()
    }
}