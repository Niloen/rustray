use std::ops::Neg;
use image::{Pixel, Rgb};
use crate::world::group::Group;
pub use crate::world::light::Light;
use crate::world::object::{HitResult, Intersecting};
pub use crate::world::object::Object;
use crate::world::ray::Ray;

pub mod ray;
mod object;
pub mod sphere;
mod group;
mod light;

pub struct World<'a> {
    root: Group<'a>,
    lights: Vec<Light>
}

fn min(v1: f64, v2: f64) -> f64 {
    if v1 < v2 { v1 } else { v2 }
}

impl<'a> World<'a> {
    pub fn new() -> World<'a> {
        World {
            root: Group::new(),
            lights: Vec::new()
        }
    }
    
    pub fn add<T: Object<'a> + 'a>(&mut self, object: T) {
        self.root.add(object);
    }
    
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    fn illumination(&self, ray: &Ray) -> Rgb<f64> {
        self.lights.iter()
            .map(|light| {
                let mut fraction = light.towards(ray.origin).direction.cos_angle(ray.direction).neg();
                if fraction < 0.0 {
                    fraction = 0.0
                }
                    
                light.color.map(|c|c * fraction)
            }).reduce(|c1,c2|c1.map2(&c2, |x1,x2|min(1.0, x1 + x2)))
            .unwrap_or_else(|| Rgb([0.0, 0.0, 0.0]))
    }
    
    pub fn cast<'b, 'c, 'z>(&'b self, ray: &'c Ray) -> Option<HitResult>
    where 'a: 'z, 'b: 'z, 'c: 'z {
        self.root.intersects(ray)
            .and_then(|i| i.object.hit(ray))
            .map(|hr| {
                let normal_ray = Ray::new(ray.at(hr.distance), hr.normal);
                
                HitResult {
                    color: self.illumination(&normal_ray).map2(&hr.color, |c1, c2| c1 * c2),
                    ..hr
                }
            })
    }
}