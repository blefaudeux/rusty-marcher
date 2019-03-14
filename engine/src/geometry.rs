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

    pub fn abs(&self) -> Vec3f {
        Vec3f {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn scaled(&self, s: f64) -> Vec3f {
        let mut other = *self;
        other.scale(s);
        other
    }

    pub fn dot(self, other: Vec3f) -> f64 {
        dot(self, other)
    }

    pub fn cross(&self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn squared_norm(self) -> f64 {
        dot(self, self)
    }

    pub fn add(&mut self, other: Vec3f) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
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
    let norm = dot(*vec, *vec).sqrt();
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
        write!(f, "{:.2} {:.2} {:.2}", self.x, self.y, self.z)
    }
}

fn dot(v1: Vec3f, v2: Vec3f) -> f64 {
    (v1.x * v2.x + v1.y * v2.y + v1.z * v2.z) as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ones() {
        let start = Vec3f::ones();
        assert![start.x == 1. && start.y == 1. && start.z == 1.];
    }

    #[test]
    fn test_scale() {
        {
            let start = Vec3f::ones();
            let start_scaled = start.scaled(1.36);
            assert![start_scaled.x == 1.36 && start_scaled.y == 1.36 && start_scaled.z == 1.36];
        }
        {
            let mut start_scale = Vec3f::ones();
            start_scale.scale(1.36);
            assert![start_scale.x == 1.36 && start_scale.y == 1.36 && start_scale.z == 1.36];
        }
    }

    #[test]
    fn test_dot() {
        {
            let a = Vec3f {
                x: 0.,
                y: 1.,
                z: 0.,
            };
            let b = Vec3f {
                x: 1.,
                y: 0.,
                z: 0.,
            };
            assert![a.dot(b) == 0.];
        }
        {
            let a = Vec3f {
                x: 0.,
                y: 1.,
                z: 0.,
            };
            let b = Vec3f {
                x: 0.,
                y: 1.,
                z: 0.,
            };
            assert![a.dot(b) == 1.];
        }
        {
            let a = Vec3f {
                x: 0.,
                y: 1.,
                z: 0.,
            };
            let b = Vec3f {
                x: 0.,
                y: -1.,
                z: 0.,
            };
            assert![a.dot(b) == -1.];
        }
        {
            let a = Vec3f {
                x: 1.,
                y: 1.,
                z: 0.,
            };
            let b = Vec3f {
                x: 1.,
                y: -1.,
                z: 0.,
            };
            assert![a.dot(b) == 0.];
        }
    }
    #[test]
    fn test_norm() {
        {
            let a = Vec3f {
                x: 42.,
                y: 1.,
                z: 0.,
            };
            assert![a.squared_norm() == 42. * 42. + 1.];
        }
        {
            let a = Vec3f {
                x: 42.,
                y: 1.,
                z: 0.,
            };
            assert![a.normalized().squared_norm() == 1.];
        }
        {
            let mut a = Vec3f {
                x: 42.,
                y: 1.,
                z: 0.,
            };
            a.normalize_l0();
            assert![a.x == 1.];
        }
    }
    #[test]
    fn test_cross() {
        {
            let a = Vec3f {
                x: 42.,
                y: 1.,
                z: 0.,
            };
            assert![a.cross(a).squared_norm() == 0.];
        }
        {
            let a = Vec3f {
                x: 42.,
                y: 1.,
                z: 0.,
            };
            assert_eq![a.cross(-a).squared_norm(), 0.];
        }
        {
            let a = Vec3f {
                x: 1.,
                y: 1.,
                z: 0.,
            };
            let b = Vec3f {
                x: 1.,
                y: -1.,
                z: 0.,
            };
            assert_eq![a.cross(b).dot(a), 0.];
        }
        {
            let a = Vec3f {
                x: 1.,
                y: 1.,
                z: 0.,
            };
            let b = Vec3f {
                x: 1.,
                y: -1.,
                z: 0.,
            };
            assert_eq![
                a.cross(b).squared_norm(),
                a.squared_norm() * b.squared_norm()
            ];
        }
        {
            let a = Vec3f {
                x: 1.,
                y: 1.,
                z: 0.,
            };
            let b = Vec3f {
                x: 1.,
                y: 0.,
                z: 0.,
            };
            println!["{}", a.cross(b).squared_norm()];
            println![
                "{}",
                a.squared_norm() * b.squared_norm() * (std::f64::consts::PI / 4.).sin().powf(2.)
            ];

            assert![
                a.cross(b).squared_norm()
                    - a.squared_norm()
                        * b.squared_norm()
                        * (std::f64::consts::PI / 4.).sin().powf(2.)
                    < 1e-4
            ];
        }
    }

    #[test]
    fn test_squared_norm() {
        let a = Vec3f {
            x: 1.,
            y: -2.,
            z: 3.,
        };
        assert_eq![a.squared_norm(), 14.];
    }

    #[test]
    fn test_add() {
        let a = Vec3f {
            x: 1.,
            y: -2.,
            z: 3.,
        };

        let b = Vec3f {
            x: 4.,
            y: -2.,
            z: 2.,
        };

        assert_eq![
            a + b,
            Vec3f {
                x: 5.,
                y: -4.,
                z: 5.
            }
        ];

        assert_eq![a - a, Vec3f::zero()];
    }
}
