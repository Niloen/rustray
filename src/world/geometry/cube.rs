use std::ops::Neg;
use crate::vector::{Point3, Vector3};
use crate::world::geometry::{HitResult, Geometry};
use crate::world::ray::Ray;
use crate::world::material::Material;
use image::Rgb;
use crate::world::intersect::{Intersecting, Intersection};
use crate::world::surface::Surface;

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
    fn normal_at(&self, hit_position: &Point3) -> Vector3 {
        let half_side = 0.5;
        let offset = *hit_position;

        // Determine the normal by finding the axis with the largest absolute value
        if offset.x.abs() > half_side - 0.001 {
            Vector3::new(offset.x.signum(), 0.0, 0.0)
        } else if offset.y.abs() > half_side - 0.001 {
            Vector3::new(0.0, offset.y.signum(), 0.0)
        } else {
            Vector3::new(0.0, 0.0, offset.z.signum())
        }
    }
}

impl Geometry for Cube {
    fn distance(&self, ray: &Ray) -> Option<f64> {
        let half_side = 0.5;

        // Calculate the min and max bounds for each axis
        let max_bound = Vector3::new(half_side, half_side, half_side);
        let min_bound = -max_bound;

        // Initialize entry and exit distances
        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;

        // Check intersection along each axis
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let t0 = (min_bound[i] - ray.origin[i]) * inv_d;
            let t1 = (max_bound[i] - ray.origin[i]) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return None; // No intersection
            }
        }

        if t_min <= 0.0 { None } else { Some(t_min) }
    }

    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.distance(ray).map(|distance| {
            let hit_position = ray.at(distance);
            HitResult {
                position: hit_position,
                normal: self.normal_at(&hit_position),
                coords: (0.0, 0.0)
            }
        })
    }
}

