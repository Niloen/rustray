use crate::algebra::{Distance, Point3, Vector3};

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn normalized(origin: Point3, direction: Vector3) -> Ray {
        Ray::new(
            origin,
            direction.normalize()
        )
    }

    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray {
            origin,
            direction,
        }
    }
    pub fn at(&self, distance: Distance) -> Point3 {
        self.origin + self.direction * distance
    }

    /// Calculates the reflected ray given a normal vector at the intersection point.
    pub fn reflect(&self, normal: Vector3) -> Ray {
        let reflected_direction = self.direction - normal * 2.0 * self.direction.dot(&normal);
        Ray::normalized(self.origin, reflected_direction)
    }    
}