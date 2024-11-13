use crate::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
    pub fn at(&self, distance: f64) -> Vector3 {
        self.origin + self.direction * distance
    }

    /// Calculates the reflected ray given a normal vector at the intersection point.
    pub fn reflect(&self, normal: Vector3) -> Ray {
        let reflected_direction = self.direction - normal * 2.0 * self.direction.dot(normal);
        Ray::new(self.origin, reflected_direction)
    }    
}