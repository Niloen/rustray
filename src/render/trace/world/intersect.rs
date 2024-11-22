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
    fn closest_intersection(&self, ray: &Ray, max: f64) -> Option<Intersection>;
    
    fn any_intersects(&self, ray: &Ray, max: f64) -> bool {
        self.closest_intersection(ray, max).is_some()
    }
}

impl Intersecting for Object {
    fn closest_intersection(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        self.distance(ray).filter(|d|*d < max).map(|distance| Intersection::new(
            distance,
            self
        ))
    }
}

impl<T: Intersecting + ?Sized> Intersecting for Arc<T> {
    fn closest_intersection(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        self.as_ref().closest_intersection(ray, max)
    }

    fn any_intersects(&self, ray: &Ray, max: f64) -> bool {
        self.as_ref().any_intersects(ray, max)
    }
}

impl<T: Intersecting> Intersecting for Vec<T> {
    fn closest_intersection(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        let mut shortest: f64 = max;
        for x in self {
            if let Some(intersection) = x.closest_intersection(ray, shortest) {
                shortest = intersection.distance;
                result.replace(intersection);
            }
        }
        result
    }
    
    fn any_intersects(&self, ray: &Ray, max: f64) -> bool {
        for x in self {
            if x.closest_intersection(ray, max).is_some() {
                return true
            }
        }
        
        false
    }
    
}

impl Intersecting for &dyn Intersecting {
    fn closest_intersection(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        (*self).closest_intersection(ray, max)
    }
    
    fn any_intersects(&self, ray: &Ray, max: f64) -> bool {
        (*self).any_intersects(ray, max)
    }
}

impl Bounded for &dyn Intersecting {
    fn bounding_box(&self) -> BoundingBox {
        return (*self).bounding_box();
    }
}

