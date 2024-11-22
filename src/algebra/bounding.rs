use std::fmt;
use std::sync::Arc;
use crate::algebra::{Point3Ops, Vector3};
use crate::algebra::{Point3, Ray};

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
            min: Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    /// Creates a new bounding box with the specified min and max corners.
    pub fn new(min: Point3, max: Point3) -> Self {
        assert!(min <= max, "BoundingBox min must be <= max");
        Self { min, max }
    }

    pub fn is_infinite(&self) -> bool {
        return self.min.iter().any(|v|*v == f64::MIN) || self.max.iter().any(|v|*v == f64::MAX); 
    }
    
    /// Expands the bounding box by a given loose factor.
    pub fn expand_by_factor(&self, factor: f64) -> Self {
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

    /// Checks if the bounding box intersects a ray.
    pub fn intersects_ray(&self, ray: &Ray) -> bool {
        let inv_dir = Vector3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let tmin = (self.min.coords - ray.origin.coords).component_mul(&inv_dir);
        let tmax = (self.max.coords - ray.origin.coords).component_mul(&inv_dir);

        let t1 = tmin.inf(&tmax); // Component-wise min
        let t2 = tmin.sup(&tmax); // Component-wise max

        let t_near = t1.max();
        let t_far = t2.min();

        t_near <= t_far && t_far >= 0.0
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