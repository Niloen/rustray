use crate::algebra::{Distance, DistanceConstants, Ray, UnitVector3};
use crate::algebra::{Bounded, BoundingBox, Point3};
use crate::scene::geometry::{Geometry, HitResult, TextureCoords};
use std::ops::Neg;

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
        }
    }


    fn texture_coords(&self, hit_position: &Point3) -> TextureCoords {
        // Assume the hit position is already on the unit sphere
        let u = 0.5 + (hit_position.z.atan2(hit_position.x) / (2.0 * Distance::PI));
        let v = 0.5 - (hit_position.y.asin() / Distance::PI);
        (u, v)
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0))
    }
}

impl Geometry for Sphere {
    fn distance(&self, ray: &Ray) -> Option<Distance> {
        // Vector from ray origin to sphere center (center is always (0,0,0))
        let origin_to_center = ray.origin.coords;

        let direction_length_squared = ray.direction.magnitude_squared();
        
        // Projection of origin_to_center onto the ray direction, in ray units
        let tca = origin_to_center.neg().dot(&ray.direction) / direction_length_squared;

        // Squared distance from sphere center to the ray, in local units
        let d2 = origin_to_center.magnitude_squared() - tca * tca * direction_length_squared;

        // If d^2 > 1, the ray misses the sphere
        if d2 > 1.0 {
            return None;
        }

        // Distance from the ray to the sphere's intersection points, in ray units
        let thc = ((1.0 - d2) / direction_length_squared).sqrt();

        // Compute the near and far intersection distances
        let t0 = tca - thc;
        let t1 = tca + thc;

        // Choose the closest positive intersection
        if t0 > 0.0 {
            Some(t0)
        } else if t1 > 0.0 {
            Some(t1)
        } else {
            None
        }
    }

    fn hit(&self, ray: &Ray, distance: Distance) -> HitResult {
        let position = ray.at(distance);
        HitResult {
            position,
            normal: UnitVector3::new_normalize(position.coords),
            coords: self.texture_coords(&position)
        }
    }
}

pub struct Sphere {
}
