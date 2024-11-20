use crate::vector::{Point3, Vector3};
use crate::world::geometry::{Geometry, HitResult, TextureCoords};
use crate::world::ray::Ray;
use crate::world::intersect::{Intersecting, Intersection};
use crate::world::texture::Texture;

impl Sphere {
    const CENTER: Point3 = Point3::new(0.0, 0.0, 0.0);
    const RADIUS: f64 = 1.0;
    
    pub fn new() -> Sphere {
        Sphere {
        }
    }


    fn texture_coords(&self, hit_position: &Point3) -> TextureCoords {
        let local_point = (*hit_position - Sphere::CENTER).normalize();
        let u = 0.5 + (local_point.z.atan2(local_point.x) / (2.0 * std::f64::consts::PI));
        let v = 0.5 - (local_point.y.asin() / std::f64::consts::PI);
        (u, v)
    }
}
impl Geometry for Sphere {
    fn distance(&self, ray: &Ray) -> Option<f64> {
        let l = Sphere::CENTER - ray.origin;
        let is_inside_sphere = l.magnitude() < Sphere::RADIUS;
        if is_inside_sphere {
            return None;
        }
        let tca = l.dot(&ray.direction);
        let d2 = l.dot(&l) - tca * tca;
        if d2 > Sphere::RADIUS * Sphere::RADIUS {
            return None;
        }
        let thc = (Sphere::RADIUS * Sphere::RADIUS - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        // Standard logic for rays originating outside the sphere
        if t0 < 0.0 {
            t0 = t1; // If t0 is negative, let's use t1 instead
            if t0 < 0.0 {
                return None; // Both t0 and t1 are negative
            }
        }
        Some(t0)
    }

    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        return self.distance(ray).map(|t0| {
            let position = ray.at(t0);
            let normal = (position - Sphere::CENTER).normalize();
            HitResult {
                position,
                normal,
                coords: self.texture_coords(&position)
            }
        })
    }
}
pub struct Sphere {
}
