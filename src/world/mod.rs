use crate::world::object::Object;
use crate::world::ray::Ray;

mod ray;
mod object;
mod sphere;

struct World<'a> {
    objects: Vec<Box<dyn Object + 'a>>,
}

impl<'a> World<'a> {
    fn new() -> World<'a> {
        World {
            objects: Vec::new(),
        }
    }
    
    fn add<T: Object + 'a>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
    
    fn intersects(&self, ray: &Ray) -> Option<&dyn Object> {
        self.objects.iter()
            .map(|o| (o.as_ref(), o.intersects(ray)))
            .min_by(|(_, d1), (_, d2)| 
            d1.partial_cmp(d2).unwrap())
            .map(|(o, _)| o)
    }
}