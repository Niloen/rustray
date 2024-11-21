use crate::algebra::Point3;
use crate::render::trace::world::intersect::Intersection;
pub use crate::scene::geometry::Geometry;
pub use crate::scene::light::Light;
pub use crate::scene::material::Material;
pub use crate::scene::object::Object;
use crate::algebra::Ray;
pub use crate::scene::ray::RayCaster;
use crate::scene::Scene;
use image::{Pixel, Rgb};
use intersect::Intersecting;
use std::sync::Arc;
use crate::render::trace::world::ltree::{Octree, OctreeConfig};

mod intersect;
mod ltree;

pub struct World {
    root: Octree,
    lights: Vec<Light>
}

fn min(v1: f64, v2: f64) -> f64 {
    if v1 < v2 { v1 } else { v2 }
}

impl World {
    pub fn new() -> World {
        World {
            root: Octree::new(OctreeConfig::new(10, 5, 1.5)),
            lights: Vec::new()
        }
    }

    pub fn from_scene(scene: &Scene) -> World {
        let mut w = World::new();
        scene.iter_objects().for_each(|obj| {
            w.add(obj);
        });
        scene.iter_lights().for_each(|light| {
            w.add_light(*light);
        });

        w
    }
    
    pub fn add(&mut self, object: &Arc<Object>) {
        self.root.add(object.clone());
    }
    
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

}

impl RayCaster for World {
    fn cast(&self, ray: &Ray, depth: u32) -> Rgb<f64>
    {
        if depth == 0 {
            return Rgb([0.0, 0.0, 0.0]);
        }

        let intersection = self.cast_intersection(ray);
        intersection
            .and_then(|i| {
                i.object.hit(ray).map(|hr| {
                    let surface = i.object.surface_at(&hr);
                    
                    surface.material.shade(ray, &hr, surface.color, self, depth)
                })
            })
            .unwrap_or(Rgb([0.0, 0.0, 0.0]))
    }

    fn direct_lightning(&self, normal_ray: &Ray) -> Rgb<f64> {
        self.lights.iter()
            .filter(|light| {
                !self.is_shadowed(normal_ray.at(0.0001), light)
            })
            .map(|light| {
                light.illuminate(normal_ray.origin, normal_ray.direction)
            }).reduce(|c1,c2|c1.map2(&c2, |x1,x2|min(1.0, x1 + x2)))
            .unwrap_or_else(|| Rgb([0.0, 0.0, 0.0]))
    }
}

impl World {
    fn is_shadowed(&self, position: Point3, light: &Light) -> bool {
        self.is_something_within_distance(&light.towards(position), light.distance_to(position))
    }

    fn is_something_within_distance(&self, ray: &Ray, distance: f64) -> bool {
        self.cast_intersection(&ray).filter(|i| {
            i.distance < distance
        }).is_some()
    }
    fn cast_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.root.intersects(ray)
    }
}

