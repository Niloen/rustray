use crate::algebra::{Distance, Ray};
use crate::algebra::{Bounded, BoundingBox, Point3, Vector3};
use crate::scene::geometry::{Geometry, HitResult, TextureCoords};

pub struct Plane {
}

impl Plane {
    pub(crate) const NORMAL: Vector3 = Vector3::new(0.0, 1.0, 0.0);

    pub fn new() -> Self {
        Self {
        }
    }

    fn uv_coordinates(&self, hit_position: &Point3) -> TextureCoords {
        (hit_position[0], hit_position[2])
    }
}

impl Bounded for Plane {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            Point3::new(Distance::MIN, 0.0, Distance::MIN), // Very large "min" corner
            Point3::new(Distance::MAX, 0.0, Distance::MAX),   // Very large "max" corner
        )
    }
}


impl Geometry for Plane {
    fn distance(&self, ray: &Ray) -> Option<Distance> {
        // Simplify the denominator calculation using NORMAL = (0, 1, 0)
        let denom = ray.direction[1];

        // If denom is close to zero, the ray is parallel to the plane
        if denom.abs() < 1e-6 {
            return None;
        }

        // Compute t using only the Y-components of POINT (0, 0, 0) and ray.origin
        let t = -ray.origin[1] / denom;

        // Ignore intersections behind the ray's origin
        if t < 0.0 {
            return None;
        }

        Some(t)
    }

    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.distance(ray).map(|distance| {
            // Calculate the hit position directly
            let position = ray.at(distance);

            HitResult {
                position,
                normal: Plane::NORMAL, // Always (0, 1, 0)
                coords: self.uv_coordinates(&position), // Texture coordinates
            }
        })
    }

}
