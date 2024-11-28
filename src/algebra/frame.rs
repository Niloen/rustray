use crate::algebra::{Matrix4, Point3, Vector3};
#[derive(Debug, Clone, Copy)]
#[repr(align(32))]
pub struct Frame {
    pub x_axis: Vector3, // Represents the first column (basis vector X)
    pub y_axis: Vector3, // Represents the second column (basis vector Y)
    pub z_axis: Vector3, // Represents the third column (basis vector Z)
    pub origin: Point3,  // Translation (position in space)
    pub scale: Vector3 // Scales of x,y,z
}

impl Frame {
    pub fn from_matrix(transform_matrix: Matrix4) -> Self {
        // Extract frame components
        let x_axis = Vector3::new(
            transform_matrix[(0, 0)],
            transform_matrix[(1, 0)],
            transform_matrix[(2, 0)],
        );
        let y_axis = Vector3::new(
            transform_matrix[(0, 1)],
            transform_matrix[(1, 1)],
            transform_matrix[(2, 1)],
        );
        let z_axis = Vector3::new(
            transform_matrix[(0, 2)],
            transform_matrix[(1, 2)],
            transform_matrix[(2, 2)],
        );
        let translation = Point3::new(
            transform_matrix[(0, 3)],
            transform_matrix[(1, 3)],
            transform_matrix[(2, 3)],
        );

        Frame::from_vectors(x_axis, y_axis, z_axis, translation)
    }

    /// Creates a Frame from rotation and translation.
    #[allow(dead_code)]
    pub fn from_rotation_translation(rotation: [Vector3; 3], translation: Point3) -> Self {
        Frame::from_vectors(rotation[0], rotation[1], rotation[2], translation)
    }

    /// Constructs a Frame from position, forward, up, and right vectors.
    #[allow(dead_code)]
    pub fn look_at(origin: Point3, forward: Vector3, up: Vector3) -> Self {
        let z_axis = forward.normalize();
        let x_axis = up.cross(&z_axis).normalize();
        let y_axis = z_axis.cross(&x_axis).normalize();

        Frame::from_vectors(x_axis, y_axis, z_axis, origin)
    }

    pub fn from_vectors(x_axis: Vector3, y_axis: Vector3, z_axis: Vector3, origin: Point3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            origin,
            scale: Vector3::new(x_axis.magnitude(), y_axis.magnitude(), z_axis.magnitude()),
        }
    }
}

impl Frame {
    /// Computes the effective scaling fcameactor for the given vector.
    /// This adjusts for non-uniform scaling when transforming distances back to world space.
    ///
    /// It is assumed that the vector v is normalized
    pub fn scale_back_along(&self, v: &Vector3) -> f64 {
        // Compute scaled vector components
        let scaled = Vector3::new(
            v.x * self.scale.x,
            v.y * self.scale.y,
            v.z * self.scale.z,
        );

        // Return the magnitude of the scaled vector
        scaled.magnitude()
    }
    /// Transforms a point to world space.
    #[inline(never)]
    pub fn transform_point(&self, point: &Point3) -> Point3 {
        self.origin + self.x_axis * point.x + self.y_axis * point.y + self.z_axis * point.z
    }
    /// Transforms a vector to world space.
    #[inline(never)]
    pub fn transform_vector(&self, vector: &Vector3) -> Vector3 {
        self.x_axis * vector.x + self.y_axis * vector.y + self.z_axis * vector.z
    }
}

impl Frame {
    /// Combines two frames: self * other.
    #[allow(dead_code)]
    pub fn combine(&self, other: &Frame) -> Self {
        Frame::from_vectors(
            self.transform_vector(&other.x_axis),
            self.transform_vector(&other.y_axis),
            self.transform_vector(&other.z_axis),
            self.transform_point(&other.origin),
        )
    }

    /// Inverts the frame (useful for local-to-world or world-to-local conversions).
    pub fn inverse(&self) -> Self {
        // Compute scale factors
        let inv_x = self.x_axis / self.x_axis.magnitude_squared();
        let inv_y = self.y_axis / self.y_axis.magnitude_squared();
        let inv_z = self.z_axis / self.z_axis.magnitude_squared();

        // Compute new origin by projecting onto the inverted axes
        let inv_origin = -Point3::new(
            self.origin.coords.dot(&inv_x),
            self.origin.coords.dot(&inv_y),
            self.origin.coords.dot(&inv_z),
        );

        Frame::from_vectors(inv_x, inv_y, inv_z, inv_origin)
    }
}
