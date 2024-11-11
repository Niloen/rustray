use crate::world::object::{Intersection, Object};
use crate::world::ray::Ray;

pub mod ray;
mod object;
pub mod sphere;

pub struct World<'a> {
    objects: Vec<Box<dyn Object + 'a>>,
}

impl<'a> World<'a> {
    pub fn new() -> World<'a> {
        World {
            objects: Vec::new(),
        }
    }
    
    pub fn add<T: Object + 'a>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
    
    pub fn closest_along(&self, ray: &Ray) -> Option<Intersection> {
        self.objects.iter()
            .map(|o|o.intersects(ray))
            .flat_map(|o| o)
            .min_by(|i1, i2| 
            i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}