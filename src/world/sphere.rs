use image::Rgb;
use crate::vector::Vector3;
use crate::world::object::{HitResult, Object, Intersection};
use crate::world::ray::Ray;

impl Sphere {
    pub fn new(center: Vector3, radius: f64, color: Rgb<f64>) -> Sphere {
        Sphere {
            center,
            radius,
            color
        }
    }
}

impl<'a> Object<'a> for  Sphere {
    fn intersects<'b, 'c, 'z>(&'b self, ray: &'c Ray) -> Option<Intersection<'z>>
    where 'a: 'z, 'b : 'z, 'c: 'z
    {
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction);
        let d2 = l.dot(l) - tca * tca;
        if d2 > self.radius * self.radius {
            return None
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;


        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        if t0 < 0.0 {
            t0 = t1; // If t0 is negative, let's use t1 instead.
            if t0 < 0.0 {
                return None; // Both t0 and t1 are negative.
            }
        }

        let center = self.center;
        Some(
            Intersection::new(
                t0,
                Box::new(move || {
                    HitResult {
                        distance: t0,
                        normal: (ray.at(t0) - center).normalize(),
                        color: self.color
                    }
                })
            ))
    }
}
#[derive(Debug)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
    color: Rgb<f64>
}