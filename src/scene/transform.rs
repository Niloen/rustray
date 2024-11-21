use nalgebra::Unit;
use crate::algebra::{Vector3, Matrix4, Point3};
use crate::algebra::Ray;

#[derive(Debug, Clone)]
pub struct Transform {
    pub matrix: Matrix4,        // Transformation matrix (world space)
    pub inverse_matrix: Matrix4, // Precomputed inverse matrix (for local space)
}

impl Transform {
    /// Creates a new transform from translation, rotation, and scale.
    pub fn new(position: Vector3, rotation: Vector3, scale: Vector3) -> Self {
        // Compute transformation matrix
        let translation = Matrix4::new_translation(&position.into());
        let rotation = Self::rotation_matrix(rotation);
        let scaling = Matrix4::new_nonuniform_scaling(&scale.into());

        let matrix = translation * rotation * scaling;
        let inverse_matrix = matrix.try_inverse().expect("Matrix must be invertible");

        Self { matrix, inverse_matrix }
    }

    /// Identity transform (no transformation)
    #[allow(dead_code)]
    pub fn identity() -> Self {
        Self {
            matrix: Matrix4::identity(),
            inverse_matrix: Matrix4::identity(),
        }
    }

    /// Applies the transform to a point in local space.
    pub fn apply_to_point(&self, point: &Point3) -> Point3 {
        self.matrix.transform_point(point)
    }

    /// Converts a ray to local space.
    pub fn to_local_ray(&self, ray: &Ray) -> Ray {
        let origin = self.inline_transform_point(&ray.origin);
        let direction = self.inline_transform_vector(&ray.direction);

        Ray::new(origin, direction)
    }

    fn inline_transform_point(&self, point: &Point3) -> Point3 {
        let p = self.inverse_matrix;
        Point3::new(
            p[(0, 0)] * point.x + p[(0, 1)] * point.y + p[(0, 2)] * point.z + p[(0, 3)],
            p[(1, 0)] * point.x + p[(1, 1)] * point.y + p[(1, 2)] * point.z + p[(1, 3)],
            p[(2, 0)] * point.x + p[(2, 1)] * point.y + p[(2, 2)] * point.z + p[(2, 3)],
        )
    }

    fn inline_transform_vector(&self, vector: &Vector3) -> Vector3 {
        let p = self.inverse_matrix;
        Vector3::new(
            p[(0, 0)] * vector.x + p[(0, 1)] * vector.y + p[(0, 2)] * vector.z,
            p[(1, 0)] * vector.x + p[(1, 1)] * vector.y + p[(1, 2)] * vector.z,
            p[(2, 0)] * vector.x + p[(2, 1)] * vector.y + p[(2, 2)] * vector.z,
        )
    }
    fn rotation_matrix(rotation: Vector3) -> Matrix4 {
        let angle = rotation.magnitude();
        if angle.abs() < 1e-6 {
            return Matrix4::identity(); // No rotation
        }

        let axis = Unit::new_normalize(rotation);
        Matrix4::from_axis_angle(&axis, angle)
    }

    pub fn scale(&self) -> f64 {
        let sx = self.matrix[(0, 0)].abs();
        let sy = self.matrix[(1, 1)].abs();
        let sz = self.matrix[(2, 2)].abs();
        sx.max(sy).max(sz) // Maximum for non-uniform scaling
    }

    fn rotation_from_axis_angle(axis: Vector3, angle: f64) -> Vector3 {
        // Compute the rotation as a quaternion or Euler angles
        // (For simplicity, this returns Euler angles; use a quaternion for more precision)
        axis * angle
    }

    pub fn rotation_to(v1: Vector3, v2: Vector3) -> Vector3 {
        let v1_normalized = v1.normalize();
        let v2_normalized = v2.normalize();

        if v1_normalized == v2_normalized {
            Vector3::zeros() // No rotation needed
        } else if v1_normalized == -v2_normalized {
            // Dynamically choose a vector not aligned with v1
            let arbitrary_axis = if v1_normalized.x.abs() < v1_normalized.y.abs() && v1_normalized.x.abs() < v1_normalized.z.abs() {
                Vector3::new(1.0, 0.0, 0.0) // Use x-axis if v1 is not aligned with x
            } else if v1_normalized.y.abs() < v1_normalized.z.abs() {
                Vector3::new(0.0, 1.0, 0.0) // Use y-axis if v1 is not aligned with y
            } else {
                Vector3::new(0.0, 0.0, 1.0) // Use z-axis otherwise
            };

            let axis = v1_normalized.cross(&arbitrary_axis).normalize(); // Ensure orthogonality
            Transform::rotation_from_axis_angle(axis, std::f64::consts::PI)
        } else {
            let axis = v2_normalized.cross(&v1_normalized).normalize();
            let angle = v2_normalized.dot(&v1_normalized).acos();
            Transform::rotation_from_axis_angle(axis, angle)
        }
    }
}
