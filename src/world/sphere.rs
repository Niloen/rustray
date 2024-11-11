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

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.hit(ray).map(Intersection::from_result)
    }
}

impl Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
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

        Some(HitResult {
            distance: t0,
            normal: (ray.at(t0) - self.center).normalize(),
            color: self.color
        })
    }
}
#[derive(Debug)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
    color: Rgb<f64>
}