use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn normalize(vec: &mut Vec3f) {
    let norm = dot(vec, vec);
    if norm > 0. {
        scale(vec, 1. / norm);
    }
}

pub fn scale(vec: &mut Vec3f, scale: f64) {
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

impl Sub for Vec3f {
    type Output = Vec3f;

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

impl std::clone::Clone for Vec3f {
    fn clone(&self) -> Vec3f {
        return Vec3f {
            x: self.x,
            y: self.y,
            z: self.z,
        };
    }
}

pub fn dot(v1: &Vec3f, v2: &Vec3f) -> f64 {
    return (v1.x * v2.x + v1.y * v2.y + v1.z * v2.z) as f64;
}
