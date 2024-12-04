use crate::algebra::{Distance, DistanceConstants, Ray, UnitVector3};
use crate::algebra::{Bounded, BoundingBox, Point3};
use crate::scene::geometry::{Geometry, HitResult, TextureCoords};

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
        // All vector operations inlined for performance, it actually performs better even though 
        // rust is inlining the dot function
        
        let origin_to_center_x = ray.origin.x;
        let origin_to_center_y = ray.origin.y;
        let origin_to_center_z = ray.origin.z;

        let direction_x = ray.direction.x;
        let direction_y = ray.direction.y;
        let direction_z = ray.direction.z;

        let direction_length_squared =
            direction_x * direction_x + direction_y * direction_y + direction_z * direction_z;

        let tca = -(origin_to_center_x * direction_x
            + origin_to_center_y * direction_y
            + origin_to_center_z * direction_z)
            / direction_length_squared;

        let d2 = origin_to_center_x * origin_to_center_x
            + origin_to_center_y * origin_to_center_y
            + origin_to_center_z * origin_to_center_z
            - tca * tca * direction_length_squared;

        if d2 > 1.0 {
            return None
        }

        let thc = ((1.0 - d2) / direction_length_squared).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

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
            coords: self.texture_coords(&position),
        }
    }
}

pub struct Sphere {}

#[cfg(test)]
mod tests {
    use crate::algebra::Vector3;
    use super::*;

    #[test]
    fn distance() {
        let sphere = Sphere::new();
        let ray = Ray::new(Point3::new(0.0, 0.0, -10.0), Vector3::new(0.0, 0.0, 2.0));
        let distance = sphere.distance(&ray).unwrap();
        assert_eq!(4.5, distance);
    }
}
