use crate::vector::Vector3;
use crate::world::object::{HitResult, Intersecting, Intersection, Object};
use crate::world::ray::Ray;
use crate::world::material::Material;
use image::Rgb;

#[derive(Debug)]
pub struct Cube<'a> {
    pub center: Vector3,
    pub side_length: f64,
    pub color: Rgb<f64>,
    pub material: Box<dyn Material + 'a>,
}

impl<'a> Cube<'a> {
    pub fn new(center: Vector3, side_length: f64, color: Rgb<f64>, material: &dyn Material) -> Self {
        Self {
            center,
            side_length,
            color,
            material: material.clone_box()
        }
    }
}

impl<'a> Cube<'a> {
    fn intersect_with_ray(&self, ray: &Ray) -> Option<f64> {
        let half_side = self.side_length / 2.0;

        // Calculate the min and max bounds for each axis
        let min_bound = self.center - Vector3::new(half_side, half_side, half_side);
        let max_bound = self.center + Vector3::new(half_side, half_side, half_side);

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

    // Compute the normal based on the intersection point
    fn normal_at(&self, hit_position: &Vector3) -> Vector3 {
        let half_side = self.side_length / 2.0;
        let offset = *hit_position - self.center;

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

impl<'a> Intersecting<'a> for Cube<'a> {
    fn intersects<'b, 'z>(&'b self, ray: &Ray) -> Option<Intersection<'z, 'a>>
    where
        'a: 'z,
        'b: 'z,
    {
        self.intersect_with_ray(ray).map(move |distance| Intersection::new(distance, self))
    }
}

impl<'a> Object<'a> for Cube<'a> {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.intersect_with_ray(ray).map(|distance| {
            let hit_position = ray.at(distance);
            HitResult {
                distance,
                color: self.color,
                position: hit_position,
                normal: self.normal_at(&hit_position),
                material: self.material.as_ref(),
            }
        })
    }
}

