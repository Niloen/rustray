mod bounding;
mod ray;
pub use ray::Ray;
pub use bounding::{BoundingBox, Bounded};
use nalgebra::Vector3 as NVector3;
use nalgebra::Matrix4 as NMatrix4;
use nalgebra::Point3 as NPoint3;

pub type Vector3 = NVector3<f64>;
pub type Matrix4 = NMatrix4<f64>;
pub type Point3 = NPoint3<f64>;

pub trait VectorOps {
    #[allow(dead_code)]
    fn perpendicular(&self) -> Vector3;

    fn cos_angle(&self, v: &Vector3) -> f64;
}

impl VectorOps for Vector3 {
    fn perpendicular(&self) -> Vector3 {
        // Choose the axis with the smallest absolute component to avoid near-parallelism
        if self.x.abs() < self.y.abs() && self.x.abs() < self.z.abs() {
            Vector3::new(1.0, 0.0, 0.0).cross(self).normalize()
        } else if self.y.abs() < self.z.abs() {
            Vector3::new(0.0, 1.0, 0.0).cross(self).normalize()
        } else {
            Vector3::new(0.0, 0.0, 1.0).cross(self).normalize()
        }
    }

    fn cos_angle(&self, v: &Vector3) -> f64 {
        self.dot(v) / (self.magnitude() * v.magnitude())
    }
}

pub trait Point3Ops {
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
}

impl Point3Ops for Point3 {
    fn min(&self, other: &Self) -> Self {
        Point3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn max(&self, other: &Self) -> Self {
        Point3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}