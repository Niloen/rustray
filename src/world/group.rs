use crate::world::intersect::{Intersecting, Intersection};
use crate::world::ray::Ray;

pub struct Group {
    objects: Vec<Box<dyn Intersecting>>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Intersecting + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Intersecting for Group {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>
    {
        let mut result: Option<Intersection> = None;
        let mut shortest: f64 = f64::MAX;
        for object in self.objects.iter() {
            if let Some(intersection) = object.intersects(ray) {
                if intersection.distance < shortest {
                    shortest = intersection.distance;
                    result.replace(intersection);
                }
            }
        }
        
        result
    }
}