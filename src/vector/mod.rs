use nalgebra::Vector3 as NVector3;
use nalgebra::Matrix4 as NMatrix4;
use nalgebra::Point3 as NPoint3;

pub type Vector3 = NVector3<f64>;
pub type Matrix4 = NMatrix4<f64>;
pub type Point3 = NPoint3<f64>;

pub trait VectorOps {
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
