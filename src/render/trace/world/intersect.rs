use crate::algebra::{Bounded, BoundingBox, Ray};
use crate::scene::geometry::Geometry;
use crate::scene::object::Object;
use std::sync::Arc;

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

pub fn closest_intersection_iter<'a, T>(
    iter: impl Iterator<Item = &'a T>,
    ray: &Ray,
    max: f64
) -> Option<Intersection<'a>>
where
    T: Intersecting + 'a,
{
    let mut result: Option<Intersection<'a>> = None;
    let mut shortest: f64 = max;
    for x in iter {
        if let Some(intersection) = x.closest_intersection(ray, shortest) {
            shortest = intersection.distance;
            result = Some(intersection);
        }
    }
    result
}

impl<'a, T: Intersecting> Intersecting for std::slice::Iter<'a, T> {
    fn closest_intersection(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        closest_intersection_iter(self.clone(), ray, max)
    }

    fn any_intersects(&self, ray: &Ray, max: f64) -> bool {
        for x in self.clone() {
            if x.closest_intersection(ray, max).is_some() {
                return true
            }
        }

        false
    }
}

impl<'a, T: Intersecting + 'a> Intersecting for Vec<T> {
    fn closest_intersection(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        closest_intersection_iter(self.iter(), ray, max)
    }

    fn any_intersects(&self, ray: &Ray, max: f64) -> bool {
        self.iter().any_intersects(ray, max)
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

