use crate::vector::{Vector3, VectorOps};
use crate::world::intersect::{Intersecting, Intersection};
use crate::world::object::{HitResult, Object};
use crate::world::ray::Ray;
use crate::world::texture::{Texture, TextureCoords};

pub struct Plane<'a> {
    pub point: Vector3,  // A point on the plane (e.g., origin of the floor)
    pub normal: Vector3, // The plane's normal vector (e.g., pointing up for a floor)
    pub texture: Box<dyn Texture<'a> + 'a>, // Texture to apply to the plane's surface
}

impl<'a> Plane<'a> {
    pub fn new(point: Vector3, normal: Vector3, texture: &dyn Texture<'a>) -> Self {
        Self {
            point,
            normal: normal.normalize(), // Ensure the normal is a unit vector
            texture: texture.clone_box(),
        }
    }

    fn uv_coordinates(&self, hit_position: &Vector3) -> TextureCoords {
        // Use the perpendicular vector to the normal for the tangent
        let tangent = self.normal.perpendicular();

        // Calculate the bitangent vector
        let bitangent = self.normal.cross(&tangent);

        // Calculate the displacement from the plane's origin point to the hit position
        let displacement = *hit_position - self.point;

        // Project the displacement onto the tangent and bitangent to get u and v coordinates
        let u = displacement.dot(&tangent);
        let v = displacement.dot(&bitangent);

        (u, v)
    }

    fn distance(&self, ray: &Ray) -> Option<f64> {
        let denom = self.normal.dot(&ray.direction);

        // If the denom is close to zero, the ray is parallel to the plane
        if denom.abs() < 1e-6 {
            return None;
        }

        // Calculate the distance to the intersection point
        let t = (self.point - ray.origin).dot(&self.normal) / denom;

        // If the intersection is behind the ray's origin, ignore it
        if t < 0.0 {
            return None;
        }

        Some(t)
    }
}

impl<'a> Intersecting<'a> for Plane<'a> {
    fn intersects<'b, 'z>(&'b self, ray: &Ray) -> Option<Intersection<'z, 'a>>
    where
        'a: 'z,
        'b: 'z,
    {
        self.distance(ray)
            .map(move |distance| Intersection::new(distance, self))
    }
}

impl<'a> Object<'a> for Plane<'a> {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.distance(ray).map(|distance| {
            // Calculate if the ray intersects the plane
            // Calculate the intersection point and normal
            let hit_position = ray.at(distance);
            let coords = self.uv_coordinates(&hit_position); // Get texture coordinates

            HitResult {
                position: hit_position,
                normal: self.normal,
                surface: self.texture.surface_at(coords),
            }
        })
    }
}
