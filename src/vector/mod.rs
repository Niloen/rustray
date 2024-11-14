use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn mul(&self, k: f64) -> Vector3 {
        Vector3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
    
    fn add(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
    
    pub fn div(&self, k: f64) -> Vector3 {
        Vector3 {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }

    pub fn sub(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    pub fn dot(&self, v: &Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub(crate) fn normalize(&self) -> Vector3 {
        self.div(self.length())
    }
    
    pub fn cos_angle(&self, v: &Vector3) -> f64 {
        self.dot(v).div(self.length() * v.length())
    }

    pub fn perpendicular(&self) -> Vector3 {
        // Choose the axis with the smallest absolute component to avoid near-parallelism
        if self.x.abs() < self.y.abs() && self.x.abs() < self.z.abs() {
            Vector3::new(1.0, 0.0, 0.0).cross(self).normalize()
        } else if self.y.abs() < self.z.abs() {
            Vector3::new(0.0, 1.0, 0.0).cross(self).normalize()
        } else {
            Vector3::new(0.0, 0.0, 1.0).cross(self).normalize()
        }
    }
}
impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::add(&self, &rhs)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::sub(&self, &rhs)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::mul(&self, rhs)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3::div(&self, rhs)
    }
}