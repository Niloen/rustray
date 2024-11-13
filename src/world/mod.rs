use image::{Pixel, Rgb};
pub use crate::world::cast::RayCaster;
use crate::world::group::Group;
pub use crate::world::light::Light;
use crate::world::object::{HitResult, Intersecting};
pub use crate::world::object::Object;
use crate::world::ray::Ray;
pub use crate::world::material::{Material, BaseMaterial};

pub mod ray;
mod object;
pub mod sphere;
mod group;
mod light;
mod material;
mod cast;

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

    fn illumination(&self, normal_ray: &Ray) -> Rgb<f64> {
        self.lights.iter()
            .map(|light| {
                light.illuminate(normal_ray.origin, normal_ray.direction)
            }).reduce(|c1,c2|c1.map2(&c2, |x1,x2|min(1.0, x1 + x2)))
            .unwrap_or_else(|| Rgb([0.0, 0.0, 0.0]))
    }
}

impl<'a> RayCaster for World<'a> {
    fn cast(&self, ray: &Ray, depth: u32) -> Rgb<f64>
    {
        if depth == 0 {
            return Rgb([0.0, 0.0, 0.0]);
        }
        
        self.root.intersects(ray)
            .and_then(|i| i.object.hit(ray))
            .map(|hr| {
                let normal_ray = Ray::new(ray.at(hr.distance), hr.normal);
                
                let color = hr.material.shade(ray, &hr, self, depth);
                
                self.illumination(&normal_ray).map2(&color, |c1, c2| c1 * c2)
            })
            .unwrap_or(Rgb([0.0, 0.0, 0.0]))
    }
}