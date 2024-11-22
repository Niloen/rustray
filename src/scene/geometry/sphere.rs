use std::ops::Neg;
use crate::algebra::{Bounded, BoundingBox, Point3};
use crate::scene::geometry::{Geometry, HitResult, TextureCoords};
use crate::algebra::Ray;

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
        }
    }


    fn texture_coords(&self, hit_position: &Point3) -> TextureCoords {
        let local_point = hit_position.coords.normalize();
        let u = 0.5 + (local_point.z.atan2(local_point.x) / (2.0 * std::f64::consts::PI));
        let v = 0.5 - (local_point.y.asin() / std::f64::consts::PI);
        (u, v)
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0))
    }
}

impl Geometry for Sphere {
    fn distance(&self, ray: &Ray) -> Option<f64> {
        // Vector from ray origin to sphere center (center is always (0,0,0))
        let origin_to_center = ray.origin.coords;

        // Projection of origin_to_center onto the ray direction
        let tca = origin_to_center.neg().dot(&ray.direction);

        // Squared distance from sphere center to the ray
        let d2 = origin_to_center.magnitude_squared() - tca * tca;

        // If d^2 > 1, the ray misses the sphere
        if d2 > 1.0 {
            return None;
        }

        // Distance from the ray to the sphere's intersection points
        let thc = (1.0 - d2).sqrt();

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

    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.distance(ray).map(|t0| {
            let position = ray.at(t0);
            let normal = -position.coords.normalize();
            HitResult {
                position,
                normal,
                coords: self.texture_coords(&position),
            }
        })
    }
}

pub struct Sphere {
}
