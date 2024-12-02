use crate::algebra::{Distance, Point3, UnitVector3, Vector3};

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Ray {
    pub origin: Point3,
    pub direction: UnitVector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray::from_normalized(
            origin,
            UnitVector3::new_normalize(direction)
        )
    }

    pub fn from_normalized(origin: Point3, direction: UnitVector3) -> Self {
        Ray {
            origin,
            direction,
        }
    }
    pub fn at(&self, distance: Distance) -> Point3 {
        self.origin + self.direction.into_inner() * distance
    }

    /// Calculates the reflected ray given a normal vector at the intersection point.
    pub fn reflect(&self, normal: Vector3) -> Ray {
        let reflected_direction = self.direction.into_inner() - normal * 2.0 * self.direction.dot(&normal);
        Ray::new(self.origin, reflected_direction)
    }    
}