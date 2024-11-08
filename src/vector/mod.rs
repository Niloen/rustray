use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
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

    pub fn add(&self, v: Vector3) -> Vector3 {
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

    pub fn sub(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    pub fn dot(&self, v: Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub(crate) fn normalize(&self) -> Vector3 {
        self.div(self.length())
    }
    
    pub fn cos_angle(&self, v: Vector3) -> f64 {
        self.dot(v).div(self.length() * v.length())
    }
}
impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::add(&self, rhs)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::sub(&self, rhs)
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