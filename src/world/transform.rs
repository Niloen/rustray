use crate::vector::{Vector3, Matrix4, Point3};
use crate::world::ray::Ray;

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

    /// Applies the inverse transform to a point (world to local).
    pub fn to_local_point(&self, point: &Point3) -> Point3 {
        self.inverse_matrix.transform_point(point)
    }

    /// Converts a ray to local space.
    pub fn to_local_ray(&self, ray: &Ray) -> Ray {
        Ray::new(
            self.to_local_point(&ray.origin),
            self.inverse_matrix.transform_vector(&ray.direction)
        )
    }

    /// Generates a rotation matrix from Euler angles.
    fn rotation_matrix(rotation: Vector3) -> Matrix4 {
        let rotation_x = Matrix4::from_euler_angles(rotation.x, 0.0, 0.0);
        let rotation_y = Matrix4::from_euler_angles(0.0, rotation.y, 0.0);
        let rotation_z = Matrix4::from_euler_angles(0.0, 0.0, rotation.z);

        rotation_z * rotation_y * rotation_x
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
        if v1 != v2 {
            let axis = v2.cross(&v1).normalize();
            let angle = v2.dot(&v1).acos();
            Transform::rotation_from_axis_angle(axis, angle)
        } else {
            Vector3::zeros() // No rotation needed
        }
    }
}
