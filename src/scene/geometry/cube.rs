use crate::algebra::{Bounded, BoundingBox, Distance, Point3, UnitVector3, Vector3};
use crate::scene::geometry::{Geometry, HitResult};
use crate::algebra::Ray;

#[derive(Debug)]
pub struct Cube {
}

impl Cube {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Cube {
    // Compute the normal based on the intersection point
    fn normal_at(&self, hit_position: &Point3) -> UnitVector3 {
        let abs_pos = hit_position.coords.map(|c| c.abs());
        let max_axis = abs_pos.imax(); // Find the dominant axis

        match max_axis {
            0 => UnitVector3::new_unchecked(Vector3::new(hit_position.x.signum(), 0.0, 0.0)), // X-axis
            1 => UnitVector3::new_unchecked(Vector3::new(0.0, hit_position.y.signum(), 0.0)), // Y-axis
            _ => UnitVector3::new_unchecked(Vector3::new(0.0, 0.0, hit_position.z.signum())), // Z-axis
        }
    }
}

impl Bounded for Cube {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(Point3::new(-0.5, -0.5, -0.5), Point3::new(0.5, 0.5, 0.5))
    }
}

impl Geometry for Cube {
    fn distance(&self, ray: &Ray) -> Option<Distance> {
        // Initialize entry and exit distances
        let mut t_min = Distance::NEG_INFINITY;
        let mut t_max = Distance::INFINITY;

        const MIN_BOUND: Distance = -0.5;
        const MAX_BOUND: Distance = 0.5;
        
        // Check intersection along each axis
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let origin = ray.origin[i];
            let t0 = (MIN_BOUND - origin) * inv_d;
            let t1 = (MAX_BOUND - origin) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return None; // No intersection
            }
        }

        if t_min <= 0.0 { None } else { Some(t_min) }
    }

    fn hit(&self, ray: &Ray, distance: Distance) -> HitResult {
        HitResult {
            position: ray.at(distance),
            normal: self.normal_at(&ray.at(distance)),
            coords: (0.0, 0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;
    
    #[test]
    #[ignore]
    fn bench() {
        let c = Cube::new();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let time = Instant::now();
        for _ in 0..10000000 {
            c.distance(&r);
        }
        println!("{:?}", time.elapsed());
    }
}

