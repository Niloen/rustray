use crate::vector::Vector3;
use crate::world::object::{HitResult, Object};
use crate::world::ray::Ray;
use crate::world::intersect::{Intersecting, Intersection};
use crate::world::texture::{Texture, TextureCoords};

impl<'a> Sphere<'a> {
    pub fn new(center: Vector3, radius: f64, texture: &dyn Texture<'a>) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            texture: texture.clone_box()
        }
    }

    fn distance(&self, ray: &Ray) -> Option<f64> {
        let l = self.center - ray.origin;
        let tca = l.dot(&ray.direction);
        let d2 = l.dot(&l) - tca * tca;
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

    fn texture_coords(&self, hit_position: &Vector3) -> TextureCoords {
        let local_point = (*hit_position - self.center).normalize();
        let u = 0.5 + (local_point.z.atan2(local_point.x) / (2.0 * std::f64::consts::PI));
        let v = 0.5 - (local_point.y.asin() / std::f64::consts::PI);
        (u, v)
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
            let normal = (position - self.center).normalize();
            HitResult {
                position,
                normal,
                surface: self.texture.surface_at(self.texture_coords(&position))
            }
        })
    }
}
pub struct Sphere<'a> {
    center: Vector3,
    radius: f64,
    texture: Box<dyn Texture<'a> + 'a>
}
