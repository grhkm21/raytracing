use impl_ops::*;
use std::{ops, fmt::Display};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x, $y, $z)
    }
}

impl Vec3 {
    // Constructors
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    // Getters
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    // Helper functions
    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    pub fn len2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.len()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Binary operators
impl_op!(+ |a: Vec3, b: Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op!(- |a: Vec3, b: Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op!(* |a: Vec3, b: f64| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });
impl_op!(/ |a: Vec3, b: f64| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });

// Assignment operators
impl_op!(+= |a: &mut Vec3, b: Vec3| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op!(-= |a: &mut Vec3, b: Vec3| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op!(*= |a: &mut Vec3, b: f64| { a.x *= b; a.y *= b; a.z *= b; });
impl_op!(/= |a: &mut Vec3, b: f64| { a.x /= b; a.y /= b; a.z /= b; });

// Unary operators
impl_op!(- |a: Vec3| -> Vec3 { Vec3::new(-a.x, -a.y, -a.z) });
