use crate::algebra::{Matrix4, Point3, Vector3};

#[derive(Debug, Clone, Copy)]
#[repr(align(32))]
pub enum Frame {
    General {
        x_axis: Vector3, // Represents the first column (basis vector X)
        y_axis: Vector3, // Represents the second column (basis vector Y)
        z_axis: Vector3, // Represents the third column (basis vector Z)
        origin: Point3,  // Translation (position in space)
    },
    Fixed {
        origin: Point3,
        scale: Vector3,
    },
}

impl Frame {
    pub fn from_matrix(transform_matrix: Matrix4) -> Self {
        // Extract scale factors from the matrix diagonal
        let scale_x = transform_matrix[(0, 0)];
        let scale_y = transform_matrix[(1, 1)];
        let scale_z = transform_matrix[(2, 2)];

        // Check if the matrix is purely a fixed transform (diagonal scaling + translation)
        let is_fixed = transform_matrix[(0, 1)] == 0.0
            && transform_matrix[(0, 2)] == 0.0
            && transform_matrix[(1, 0)] == 0.0
            && transform_matrix[(1, 2)] == 0.0
            && transform_matrix[(2, 0)] == 0.0
            && transform_matrix[(2, 1)] == 0.0
            && scale_x > 0.0
            && scale_y > 0.0
            && scale_z > 0.0;

        if is_fixed {
            let origin = Point3::new(
                transform_matrix[(0, 3)],
                transform_matrix[(1, 3)],
                transform_matrix[(2, 3)],
            );
            let scale = Vector3::new(scale_x, scale_y, scale_z);

            Frame::Fixed { origin, scale }
        } else {
            // Extract basis vectors
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
            let origin = Point3::new(
                transform_matrix[(0, 3)],
                transform_matrix[(1, 3)],
                transform_matrix[(2, 3)],
            );

            Frame::General {
                x_axis,
                y_axis,
                z_axis,
                origin,
            }
        }
    }

    /// Constructs a Frame from position, forward, up, and right vectors.
    #[allow(dead_code)]
    pub fn look_at(origin: Point3, forward: Vector3, up: Vector3) -> Self {
        let z_axis = forward.normalize();
        let x_axis = up.cross(&z_axis).normalize();
        let y_axis = z_axis.cross(&x_axis).normalize();

        Frame::General {
            x_axis,
            y_axis,
            z_axis,
            origin,
        }
    }

    /// Transforms a point to world space.
    #[inline(never)]
    pub fn transform_point(&self, point: &Point3) -> Point3 {
        match self {
            Frame::General { x_axis, y_axis, z_axis, origin } => {
                *origin + *x_axis * point.x + *y_axis * point.y + *z_axis * point.z
            }
            Frame::Fixed { origin, scale } => {
                Point3::new(
                    origin.x + point.x * scale.x,
                    origin.y + point.y * scale.y,
                    origin.z + point.z * scale.z,
                )
            }
        }
    }

    /// Transforms a vector to world space.
    #[inline(never)]
    pub fn transform_vector(&self, vector: &Vector3) -> Vector3 {
        match self {
            Frame::General { x_axis, y_axis, z_axis, .. } => {
                *x_axis * vector.x + *y_axis * vector.y + *z_axis * vector.z
            }
            Frame::Fixed { scale, .. } => {
                Vector3::new(
                    vector.x * scale.x,
                    vector.y * scale.y,
                    vector.z * scale.z,
                )
            }
        }
    }

    /// Combines two frames: self * other.
    #[allow(dead_code)]
    pub fn combine(&self, other: &Frame) -> Self {
        match self {
            Frame::General { x_axis, y_axis, z_axis, origin: _origin } => match other {
                Frame::General { .. } => Frame::General {
                    x_axis: self.transform_vector(&other.get_x_axis()),
                    y_axis: self.transform_vector(&other.get_y_axis()),
                    z_axis: self.transform_vector(&other.get_z_axis()),
                    origin: self.transform_point(&other.get_origin()),
                },
                Frame::Fixed { scale, origin: other_origin } => Frame::Fixed {
                    origin: self.transform_point(other_origin),
                    scale: Vector3::new(
                        x_axis.magnitude() * scale.x,
                        y_axis.magnitude() * scale.y,
                        z_axis.magnitude() * scale.z,
                    ),
                },
            },
            Frame::Fixed { origin, scale } => match other {
                Frame::General { .. } => Frame::General {
                    x_axis: Vector3::new(scale.x, 0.0, 0.0),
                    y_axis: Vector3::new(0.0, scale.y, 0.0),
                    z_axis: Vector3::new(0.0, 0.0, scale.z),
                    origin: origin + other.get_origin().coords,
                },
                Frame::Fixed { scale: other_scale, origin: other_origin } => Frame::Fixed {
                    origin: Point3::new(
                        origin.x + other_origin.x * scale.x,
                        origin.y + other_origin.y * scale.y,
                        origin.z + other_origin.z * scale.z,
                    ),
                    scale: Vector3::new(
                        scale.x * other_scale.x,
                        scale.y * other_scale.y,
                        scale.z * other_scale.z,
                    ),
                },
            },
        }
    }

    /// Inverts the frame (useful for local-to-world or world-to-local conversions).
    pub fn inverse(&self) -> Self {
        match self {
            Frame::General { x_axis, y_axis, z_axis, origin } => {
                let inv_x = *x_axis / x_axis.magnitude_squared();
                let inv_y = *y_axis / y_axis.magnitude_squared();
                let inv_z = *z_axis / z_axis.magnitude_squared();

                let inv_origin = -Point3::new(
                    origin.coords.dot(&inv_x),
                    origin.coords.dot(&inv_y),
                    origin.coords.dot(&inv_z),
                );

                Frame::General {
                    x_axis: inv_x,
                    y_axis: inv_y,
                    z_axis: inv_z,
                    origin: inv_origin,
                }
            }
            Frame::Fixed { origin, scale } => Frame::Fixed {
                origin: Point3::new(-origin.x / scale.x, -origin.y / scale.y, -origin.z / scale.z),
                scale: Vector3::new(1.0 / scale.x, 1.0 / scale.y, 1.0 / scale.z),
            },
        }
    }

    /// Helper method to retrieve the x-axis vector for Frame::General
    fn get_x_axis(&self) -> Vector3 {
        match self {
            Frame::General { x_axis, .. } => *x_axis,
            Frame::Fixed { .. } => Vector3::new(1.0, 0.0, 0.0),
        }
    }

    /// Helper method to retrieve the y-axis vector for Frame::General
    fn get_y_axis(&self) -> Vector3 {
        match self {
            Frame::General { y_axis, .. } => *y_axis,
            Frame::Fixed { .. } => Vector3::new(0.0, 1.0, 0.0),
        }
    }

    /// Helper method to retrieve the z-axis vector for Frame::General
    fn get_z_axis(&self) -> Vector3 {
        match self {
            Frame::General { z_axis, .. } => *z_axis,
            Frame::Fixed { .. } => Vector3::new(0.0, 0.0, 1.0),
        }
    }

    /// Helper method to retrieve the origin for both variants
    fn get_origin(&self) -> Point3 {
        match self {
            Frame::General { origin, .. } => *origin,
            Frame::Fixed { origin, .. } => *origin,
        }
    }
}
