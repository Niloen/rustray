use crate::world::group::Group;
use crate::world::object::{HitResult, Intersecting};
pub use crate::world::object::Object;
use crate::world::ray::Ray;

pub mod ray;
mod object;
pub mod sphere;
mod group;

pub struct World<'a> {
    root: Group<'a>
}

impl<'a> World<'a> {
    pub fn new() -> World<'a> {
        World {
            root: Group::new()
        }
    }
    
    pub fn add<T: Object<'a> + 'a>(&mut self, object: T) {
        self.root.add(object);
    }

    pub fn closest_along<'b, 'c, 'z>(&'b self, ray: &'c Ray) -> Option<HitResult>
    where 'a: 'z, 'b: 'z, 'c: 'z {
        self.root.intersects(ray)
            .and_then(|i| i.object.hit(ray))
    }
}