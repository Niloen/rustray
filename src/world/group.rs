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
        let obj = &self.objects;
        obj.iter()
            .map(|o|o.intersects(ray))
            .flat_map(|o| o)
            .min_by(|i1, i2|
                i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}