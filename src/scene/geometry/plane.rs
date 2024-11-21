use crate::algebra::{Point3, Vector3, VectorOps};
use crate::scene::geometry::{Geometry, HitResult, TextureCoords};
use crate::algebra::Ray;

pub struct Plane {
    tangent: Vector3,
    bi_tangent: Vector3,
}

impl Plane {
    pub(crate) const NORMAL: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    const POINT: Point3 = Point3::new(0.0, 0.0, 0.0);
    
    pub fn new() -> Self {
        let tangent = Plane::NORMAL.perpendicular();
        let bi_tangent = Plane::NORMAL.cross(&tangent);

        Self {
            tangent,
            bi_tangent,
        }
    }

    fn uv_coordinates(&self, hit_position: &Point3) -> TextureCoords {
        let displacement = hit_position - Plane::POINT;
        
        // Project the displacement onto the tangent and bitangent
        let u = displacement.dot(&self.tangent);
        let v = displacement.dot(&self.bi_tangent);

        (u, v)
    }
}

impl Geometry for Plane {
    fn distance(&self, ray: &Ray) -> Option<f64> {
        let denom = Plane::NORMAL.dot(&ray.direction);

        // If the denom is close to zero, the ray is parallel to the plane
        if denom.abs() < 1e-6 {
            return None;
        }

        // Calculate the distance to the intersection point
        let t = (Plane::POINT - ray.origin).dot(&Plane::NORMAL) / denom;

        // If the intersection is behind the ray's origin, ignore it
        if t < 0.0 {
            return None;
        }

        Some(t)
    }

    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.distance(ray).map(|distance| {
            // Calculate if the ray intersects the plane
            // Calculate the intersection point and normal
            let position = ray.at(distance);
            let coords = self.uv_coordinates(&position); // Get texture coordinates

            HitResult {
                position,
                normal: Plane::NORMAL,
                coords
            }
        })
    }

}
