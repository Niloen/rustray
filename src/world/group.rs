use crate::world::Object;
use crate::world::object::{Intersecting, Intersection};
use crate::world::ray::Ray;

pub struct Group<'a> {
    objects: Vec<Box<dyn Intersecting<'a> + 'a>>,
}

impl<'a> Group<'a> {
    pub fn new() -> Group<'a> {
        Group {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Object<'a> + 'a>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl<'a> Intersecting<'a> for Group<'a> {
    fn intersects<'b, 'z>(&'b self, ray: &Ray) -> Option<Intersection<'z, 'a>>
    where
        'a: 'z,
        'b: 'z
    {
        let mut result: Option<Intersection> = None;
        let mut shortest: f64 = f64::MAX;
        for object in self.objects.iter() {
            if let Some(intersection) = object.intersects(ray) {
                if (intersection.distance < shortest) {
                    shortest = intersection.distance;
                    result.replace(intersection);
                }
            }
        }
        
        result
    }
}