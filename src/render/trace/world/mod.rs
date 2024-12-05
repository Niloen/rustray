use crate::algebra::{Distance, DistanceConstants, Point3, UnitVector3};
use crate::algebra::Ray;
use crate::render::trace::world::intersect::Intersection;
use crate::render::trace::world::otree::{Octree, OctreeConfig};
pub use crate::scene::geometry::Geometry;
pub use crate::scene::light::Light;
pub use crate::scene::ray::RayCaster;
use crate::scene::{Color, ColorPart, Scene};
use image::{Pixel, Rgb};
use intersect::Intersecting;
use std::sync::Arc;

mod intersect;
mod otree;

pub struct World {
    root: Box<dyn Intersecting>,
    //root: Vec<Arc<dyn Intersecting>>,
    lights: Vec<Light>,
}

fn min(v1: ColorPart, v2: ColorPart) -> ColorPart {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}

impl World {
    pub fn new(objects: Vec<Arc<dyn Intersecting>>) -> World {
        World {
            root: if objects.len() > 20 {
                Box::new(Octree::new(OctreeConfig::new(8, 5, 1.5), objects))
            } else {
                Box::new(objects)
            },
            lights: Vec::new(),
        }
    }

    pub fn from_scene(scene: &Scene) -> World {
        let mut w = World::new(
            scene
                .iter_objects()
                .map(|o| o.clone() as Arc<dyn Intersecting>)
                .collect(),
        );

        scene.iter_lights().for_each(|light| {
            w.add_light(*light);
        });

        w
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}

impl RayCaster for World {
    fn cast(&self, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return Rgb([0.0, 0.0, 0.0]);
        }

        let intersection = self.cast_intersection(ray);
        intersection
            .map(|i| {
                let hr = i.object.hit(ray, i.distance);
                let surface = i.object.surface_at(&hr);

                surface.material.shade(ray, &hr, surface.color, self, depth)
            })
            .unwrap_or(Rgb([0.0, 0.0, 0.0]))
    }

    fn direct_lightning(&self, position: &Point3, normal: &UnitVector3) -> Color {
        let mut c = Color::from([0.0 as ColorPart, 0.0, 0.0]);

        for l in self.lights.iter() {
            let color = l.illuminate(*position, *normal);
            if color != World::BLACK {
                let ray = Ray::new(*position, normal.into_inner());
                if !self.is_shadowed(ray.at(Distance::OFF_SURFACE), l) {
                    c = c.map2(&color, |x1, x2| min(1.0, x1 + x2))
                }
            }
        }

        c
    }
}

impl World {
    const BLACK: Color = Rgb([0.0, 0.0, 0.0]);

    fn is_shadowed(&self, position: Point3, light: &Light) -> bool {
        self.is_something_within_distance(&light.towards(position), 1.0)
    }

    fn is_something_within_distance(&self, ray: &Ray, distance: Distance) -> bool {
        self.root.any_intersects(&ray, distance)
    }
    fn cast_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.root.closest_intersection(ray, Distance::MAX)
    }
}
