use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Ray {
    pub orig: Vec3f,
    pub dir: Vec3f,
    pub hit_number: u8,
}

#[allow(dead_code)]
impl Vec3f {
    pub fn normalized(&self) -> Vec3f {
        let mut other = *self;
        normalize(&mut other);
        other
    }

    pub fn normalize_l0(&mut self) {
        normalize_l0(self);
    }

    pub fn normalized_l0(&self) -> Vec3f {
        let mut other = *self;
        other.normalize_l0();
        other
    }

    pub fn scale(&mut self, s: f64) {
        scale(self, s);
    }

    pub fn scaled(&self, s: f64) -> Vec3f {
        let mut other = *self;
        other.scale(s);
        other
    }

    pub fn dot(&self, other: &Vec3f) -> f64 {
        dot(self, other)
    }

    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn squared_norm(&self) -> f64 {
        dot(self, self)
    }

    pub fn max(&self) -> f64 {
        f64::max(f64::max(self.x, self.y), self.z)
    }

    pub fn min(&self) -> f64 {
        f64::min(f64::max(self.x, self.y), self.z)
    }

    // Common values
    pub fn zero() -> Vec3f {
        Vec3f {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn ones() -> Vec3f {
        Vec3f {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }
}

// Some of the implementations, private
fn normalize(vec: &mut Vec3f) {
    let norm = dot(vec, vec).sqrt();
    if norm > 0. {
        scale(vec, 1. / norm);
    }
}

fn normalize_l0(vec: &mut Vec3f) {
    let norm = vec.x.max(vec.y).max(vec.z);
    if norm > 0. {
        scale(vec, 1. / norm);
    }
}

fn scale(vec: &mut Vec3f, scale: f64) {
    vec.x *= scale;
    vec.y *= scale;
    vec.z *= scale;
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Vec3f {
        Vec3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3f {
    fn add_assign(&mut self, other: Vec3f) {
        *self = Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul for Vec3f {
    type Output = Self;
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Self;
    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::fmt::Display for Vec3f {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

fn dot(v1: &Vec3f, v2: &Vec3f) -> f64 {
    (v1.x * v2.x + v1.y * v2.y + v1.z * v2.z) as f64
}
