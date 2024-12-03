use crate::algebra::{Distance, DistanceConstants, Ray, UnitVector3};
use crate::algebra::{Frame, Matrix4, Point3, Vector3};
use nalgebra::Unit;

#[derive(Debug, Clone)]
pub struct Transform {
    matrix: Frame,         // Transformation matrix (world space)
    inverse_matrix: Frame, // Precomputed inverse matrix (for local space)
}

impl Transform {
}

impl Transform {
    /// Creates a new transform from translation, rotation, and scale.
    pub fn new(position: Vector3, rotation: Vector3, scale: Vector3) -> Self {
        // Compute transformation matrix
        let translation = Matrix4::new_translation(&position.into());
        let rotation = Self::rotation_matrix(rotation);
        let scaling = Matrix4::new_nonuniform_scaling(&scale.into());

        let matrix = translation * rotation * scaling;
        let frame = Frame::from_matrix(matrix);

        Self {
            matrix: frame,
            inverse_matrix: frame.inverse()
        }
    }

    /// Applies the transform to a point in local space.
    pub fn apply_to_point(&self, point: &Point3) -> Point3 {
        self.matrix.transform_point(point)
    }

    pub(crate) fn apply_to_vector(&self, v: &Vector3) -> Vector3 {
        self.matrix.transform_vector(v)
    }

    pub fn to_local_ray(&self, ray: &Ray) -> Ray {
        Ray::new(
            self.inverse_matrix.transform_point(&ray.origin),
            self.inverse_matrix.transform_vector(&ray.direction),
        )
    }

    pub fn to_local_ray_unnormalized(&self, ray: &Ray) -> Ray {
        Ray::from_normalized(
            self.inverse_matrix.transform_point(&ray.origin),
            UnitVector3::new_unchecked(self.inverse_matrix.transform_vector(&ray.direction)),
        )
    }

    pub fn to_world_distance(&self, ray: &Ray, distance: Distance) -> Distance {
        distance / self.inverse_matrix.scale_back_along(&ray.direction)
    }

    pub fn to_local_distance(&self, ray: &Ray, distance: Distance) -> Distance {
        distance * self.inverse_matrix.scale_back_along(&ray.direction)
    }

    fn rotation_matrix(rotation: Vector3) -> Matrix4 {
        let angle = rotation.magnitude();
        if angle.abs() < 1e-6 {
            return Matrix4::identity(); // No rotation
        }

        let axis = Unit::new_normalize(rotation);
        Matrix4::from_axis_angle(&axis, angle)
    }

    fn rotation_from_axis_angle(axis: Vector3, angle: Distance) -> Vector3 {
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
            let arbitrary_axis = if v1_normalized.x.abs() < v1_normalized.y.abs()
                && v1_normalized.x.abs() < v1_normalized.z.abs()
            {
                Vector3::new(1.0, 0.0, 0.0) // Use x-axis if v1 is not aligned with x
            } else if v1_normalized.y.abs() < v1_normalized.z.abs() {
                Vector3::new(0.0, 1.0, 0.0) // Use y-axis if v1 is not aligned with y
            } else {
                Vector3::new(0.0, 0.0, 1.0) // Use z-axis otherwise
            };

            let axis = v1_normalized.cross(&arbitrary_axis).normalize(); // Ensure orthogonality
            Transform::rotation_from_axis_angle(axis, Distance::PI)
        } else {
            let axis = v2_normalized.cross(&v1_normalized).normalize();
            let angle = v2_normalized.dot(&v1_normalized).acos();
            Transform::rotation_from_axis_angle(axis, angle)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_local() {
        let t = Transform::new(Vector3::new(5.0, 2.0, 2.0), Vector3::zeros(), Vector3::new(2.0, 1.0, 1.0));

        let ray = Ray::new(Point3::origin(), Vector3::new(1.0, 1.0, 1.0));
        let local_ray = t.to_local_ray(&ray);
        
        assert_eq!(local_ray.origin, Point3::new(-2.5, -2.0, -2.0));
        assert_eq!(local_ray.direction.into_inner(), Vector3::new(0.3333333333333333, 0.6666666666666666, 0.6666666666666666));
        
        assert_eq!(t.to_world_distance(&ray, 1.0), 1.1547005383792512);
    }
    
}