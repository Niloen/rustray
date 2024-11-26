use crate::algebra::{Matrix4, Point3, Ray, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub x_axis: Vector3, // Represents the first column (basis vector X)
    pub y_axis: Vector3, // Represents the second column (basis vector Y)
    pub z_axis: Vector3, // Represents the third column (basis vector Z)
    pub origin: Point3,  // Translation (position in space)
}

impl Frame {
    pub fn from_matrix(transform_matrix: Matrix4) -> Self {
        // Extract frame components
        let x_axis = Vector3::new(transform_matrix[(0, 0)], transform_matrix[(1, 0)], transform_matrix[(2, 0)]);
        let y_axis = Vector3::new(transform_matrix[(0, 1)], transform_matrix[(1, 1)], transform_matrix[(2, 1)]);
        let z_axis = Vector3::new(transform_matrix[(0, 2)], transform_matrix[(1, 2)], transform_matrix[(2, 2)]);
        let translation = Point3::new(transform_matrix[(0, 3)], transform_matrix[(1, 3)], transform_matrix[(2, 3)]);

        Frame {
            x_axis,
            y_axis,
            z_axis,
            origin: translation,
        }
    }
    
    /// Creates an identity Frame (no rotation, scaling, or translation).
    pub fn identity() -> Self {
        Self {
            x_axis: Vector3::new(1.0, 0.0, 0.0),
            y_axis: Vector3::new(0.0, 1.0, 0.0),
            z_axis: Vector3::new(0.0, 0.0, 1.0),
            origin: Point3::origin()
        }
    }

    /// Creates a Frame from rotation and translation.
    pub fn from_rotation_translation(rotation: [Vector3; 3], translation: Point3) -> Self {
        Self {
            x_axis: rotation[0],
            y_axis: rotation[1],
            z_axis: rotation[2],
            origin: translation,
        }
    }

    /// Constructs a Frame from position, forward, up, and right vectors.
    pub fn look_at(origin: Point3, forward: Vector3, up: Vector3) -> Self {
        let z_axis = forward.normalize();
        let x_axis = up.cross(&z_axis).normalize();
        let y_axis = z_axis.cross(&x_axis).normalize();
        Self {
            x_axis,
            y_axis,
            z_axis,
            origin,
        }
    }
}

impl Frame {
    pub(crate) fn scale(&self) -> f64 {
        let scale_x = self.x_axis.magnitude();
        let scale_y = self.y_axis.magnitude();
        let scale_z = self.z_axis.magnitude();

        // Return the average scale, but you can choose min or max if needed.
        (scale_x + scale_y + scale_z) / 3.0
    }

    /// Transforms a point to world space.
    pub fn transform_point(&self, point: &Point3) -> Point3 {
        self.origin
            + self.x_axis * point.x
            + self.y_axis * point.y
            + self.z_axis * point.z
    }

    /// Transforms a vector to world space.
    pub fn transform_vector(&self, vector: &Vector3) -> Vector3 {
        self.x_axis * vector.x + self.y_axis * vector.y + self.z_axis * vector.z
    }

    /// Transforms a ray to world space.
    pub fn transform_ray(&self, ray: &Ray) -> Ray {
        Ray::new(
            self.transform_point(&ray.origin),
            self.transform_vector(&ray.direction)
        )
    }
}

impl Frame {
    /// Combines two frames: self * other.
    pub fn combine(&self, other: &Frame) -> Self {
        Self {
            x_axis: self.transform_vector(&other.x_axis),
            y_axis: self.transform_vector(&other.y_axis),
            z_axis: self.transform_vector(&other.z_axis),
            origin: self.transform_point(&other.origin),
        }
    }

    /// Inverts the frame (useful for local-to-world or world-to-local conversions).
    pub fn inverse(&self) -> Self {
        let inv_x = self.x_axis.normalize();
        let inv_y = self.y_axis.normalize();
        let inv_z = self.z_axis.normalize();
        let inv_origin = -Point3::new(
            self.origin.coords.dot(&inv_x),
            self.origin.coords.dot(&inv_y),
            self.origin.coords.dot(&inv_z),
        );
        Self {
            x_axis: inv_x,
            y_axis: inv_y,
            z_axis: inv_z,
            origin: inv_origin,
        }
    }
}
