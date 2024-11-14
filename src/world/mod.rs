pub use crate::world::cast::RayCaster;
use crate::world::group::Group;
pub use crate::world::light::Light;
pub use crate::world::material::{BaseMaterial, Material};
pub use crate::world::object::Object;
pub use crate::world::surface::Surface;
use crate::world::ray::Ray;
use image::{Pixel, Rgb};
use intersect::Intersecting;
use crate::vector::Vector3;
use crate::world::intersect::Intersection;

pub mod ray;
pub mod object;
mod group;
mod light;
mod material;
mod cast;
pub mod texture;
mod surface;
mod intersect;

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

}

impl<'a> RayCaster for World<'a> {
    fn cast(&self, ray: &Ray, depth: u32) -> Rgb<f64>
    {
        if depth == 0 {
            return Rgb([0.0, 0.0, 0.0]);
        }

        let intersection = self.cast_intersection(ray);
        intersection
            .and_then(|i| i.object.hit(ray))
            .map(|hr| hr.surface.material.shade(ray, &hr, self, depth))
            .unwrap_or(Rgb([0.0, 0.0, 0.0]))
    }

    fn direct_lightning(&self, normal_ray: &Ray) -> Rgb<f64> {
        self.lights.iter()
            .filter(|light| {
                !self.is_shadowed(normal_ray.at(41.0), light)
            })
            .map(|light| {
                light.illuminate(normal_ray.origin, normal_ray.direction)
            }).reduce(|c1,c2|c1.map2(&c2, |x1,x2|min(1.0, x1 + x2)))
            .unwrap_or_else(|| Rgb([0.0, 0.0, 0.0]))
    }
}

impl<'a> World<'a> {
    fn is_shadowed(&self, position: Vector3, light: &Light) -> bool {
        self.is_something_within_distance(&light.towards(position), light.distance_to(position))
    }
}

impl<'a> World<'a> {
    fn is_something_within_distance(&self, ray: &Ray, distance: f64) -> bool {
        self.cast_intersection(&ray).filter(|i| {
            i.distance < distance
        }).is_some()
    }
}

impl<'a> World<'a> {
    fn cast_intersection<'b>(&'b self, ray: &Ray) -> Option<Intersection<'b, 'a>> {
        self.root.intersects(ray)
    }
}

