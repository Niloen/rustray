use crate::world::object::Object;
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
    
    pub fn closest_along(&self, ray: &Ray) -> Option<&dyn Object> {
        self.objects.iter()
            .map(|o| (o.as_ref(), o.intersects(ray)))
            .filter(|(_, d)| d.is_some())
            .min_by(|(_, d1), (_, d2)| 
            d1.partial_cmp(d2).unwrap())
            .map(|(o, _)| o)
    }
}