use crate::vector::Vector3;
use crate::world::object::{HitResult, Intersecting, Intersection, Object};
use crate::world::ray::Ray;
use image::Rgb;
use crate::world::material::Material;

impl<'a> Sphere<'a> {
    pub fn new(center: Vector3, radius: f64, color: Rgb<f64>, material: &dyn Material) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            color,
            material: material.clone_box()
        }
    }

    fn distance(&self, ray: &Ray) -> Option<f64> {
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction);
        let d2 = l.dot(l) - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
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
        Some(t0)
    }
}
impl<'a> Intersecting<'a> for Sphere<'a> {
    fn intersects<'b, 'z>(&'b self, ray: &Ray) -> Option<Intersection<'z, 'a>>
    where
        'a: 'z,
        'b: 'z,
    {
        self.distance(ray).map(move |distance| Intersection::new(distance, self))
    }
}

impl<'a> Object<'a> for Sphere<'a> {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        return self.distance(ray).map(|t0| {
            let position = ray.at(t0);
            HitResult {
                color: self.color,
                position,
                normal: (position - self.center).normalize(),
                material: self.material.as_ref()
            }
        })
    }
}
#[derive(Debug)]
pub struct Sphere<'a> {
    center: Vector3,
    radius: f64,
    color: Rgb<f64>,
    material: Box<dyn Material + 'a>
}
