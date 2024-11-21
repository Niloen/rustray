use std::sync::Arc;
use gtk4::prelude::GestureExt;
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

    pub fn min(&self, other: &'a Intersection) -> &Intersection {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }
}

pub trait Intersecting: Send + Sync + Bounded {
    fn intersects(&self, ray: &Ray, max: f64) -> Option<Intersection>;
}

impl Intersecting for Object {
    fn intersects(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        self.distance(ray).filter(|d|*d < max).map(|distance| Intersection::new(
            distance,
            self
        ))
    }
}

impl<T: Intersecting + ?Sized> Intersecting for Arc<T> {
    fn intersects(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        self.as_ref().intersects(ray, max)
    }
}

impl<T: Intersecting> Intersecting for Vec<T> {
    fn intersects(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        let mut shortest: f64 = max;
        for x in self {
            if let Some(intersection) = x.intersects(ray, shortest) {
                shortest = intersection.distance;
                result.replace(intersection);
            }
        }
        result
    }
}

impl Intersecting for &dyn Intersecting {
    fn intersects(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        (*self).intersects(ray, max)
    }
}

impl Bounded for &dyn Intersecting {
    fn bounding_box(&self) -> BoundingBox {
        return (*self).bounding_box();
    }
}

