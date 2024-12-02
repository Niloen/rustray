use crate::algebra::{Distance, Point3, Ray};
use crate::algebra::Point3Ops;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Point3,
    pub max: Point3,
}

impl BoundingBox {
    /// Creates an empty bounding box.
    /// This is useful for initializing a bounding box that will be expanded later.
    pub fn empty() -> Self {
        Self {
            min: Point3::new(Distance::INFINITY, Distance::INFINITY, Distance::INFINITY),
            max: Point3::new(Distance::NEG_INFINITY, Distance::NEG_INFINITY, Distance::NEG_INFINITY),
        }
    }

    /// Creates a new bounding box with the specified min and max corners.
    pub fn new(min: Point3, max: Point3) -> Self {
        assert!(min <= max, "BoundingBox min must be <= max");
        Self { min, max }
    }

    pub fn is_infinite(&self) -> bool {
        return self.min.iter().any(|v|*v == Distance::MIN) || self.max.iter().any(|v|*v == Distance::MAX);
    }
    
    /// Expands the bounding box by a given loose factor.
    pub fn expand_by_factor(&self, factor: Distance) -> Self {
        let center = self.center();
        let half_size = (self.max - self.min) * 0.5 * factor;

        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    /// Returns the union of this bounding box with another, expanding to contain both.
    pub fn union(&self, other: &BoundingBox) -> Self {
        Self {
            min: self.min.min(&other.min),
            max: self.max.max(&other.max),
        }
    }

    /// Checks if this bounding box contains another bounding box fully.
    pub fn contains(&self, other: &BoundingBox) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    /// Checks if the bounding box intersects a ray within max distance
    pub fn intersects_ray(&self, ray: &Ray, max: Distance) -> bool {
        let mut t_near = Distance::NEG_INFINITY;
        let mut t_far = Distance::INFINITY;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let t1 = (self.max[i] - ray.origin[i]) * inv_d;

            // Ensure t0 is the min and t1 is the max
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            t_near = t_near.max(t0);
            t_far = t_far.min(t1);

            // Early exit: If t_near is greater than t_far, there's no intersection
            if t_near > t_far {
                return false;
            }
        }

        // Ensure the intersection is in the positive ray direction
        t_far >= 0.0 && t_far <= max
    }

    /// Computes the center point of the bounding box.
    pub fn center(&self) -> Point3 {
        Point3::from((self.min.coords + self.max.coords) * 0.5)
    }

    /// Subdivides the bounding box into one of its 8 child octants based on the index.
    pub fn subdivide(&self, index: usize, center: Point3) -> Self {
        let mut min = self.min;
        let mut max = self.max;

        if index & 1 == 0 {
            max.x = center.x;
        } else {
            min.x = center.x;
        }

        if index & 2 == 0 {
            max.y = center.y;
        } else {
            min.y = center.y;
        }

        if index & 4 == 0 {
            max.z = center.z;
        } else {
            min.z = center.z;
        }

        Self { min, max }
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BoundingBox(min: {:?}, max: {:?})",
            self.min, self.max
        )
    }
}

pub trait Bounded {
    fn bounding_box(&self) -> BoundingBox;
}

impl<T: Bounded + ?Sized> Bounded for Arc<T> {
    fn bounding_box(&self) -> BoundingBox {
        self.as_ref().bounding_box()
    }
}

impl<T: Bounded> Bounded for Vec<T> {
    fn bounding_box(&self) -> BoundingBox {
        self
            .iter()
            .bounding_box()
    }
}

impl<'a, T: Bounded> Bounded for std::slice::Iter<'a, T> {
    fn bounding_box(&self) -> BoundingBox {
        self.clone() // Clone the iterator to consume it
            .map(|obj| obj.bounding_box())
            .fold(BoundingBox::empty(), |a, b| a.union(&b))
    }
}