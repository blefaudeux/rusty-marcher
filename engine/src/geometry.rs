use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
struct Vec3f {
    x : f64,
    y: f64,
    z: f64
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, other:Vec3f) -> Vec3f {
        Vec3f {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other:Vec3f) -> Vec3f {
        Vec3f {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl std::fmt::Display for Vec3f {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
       write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

fn dot( v1: &Vec3f, v2: &Vec3f) -> f64 {
    let dotProduct = (v1.x * v2.x + v1.y * v2.y + v1.z * v2.z);
}

fn main() {
    print!("{}", Vec3f {x: 1., y: 0., z:8.} + Vec3f {x: 2., y: 3., z:2.});

    let v1 = Vec3f {x: 1., y: 0., z:8.};
    let v2 = Vec3f {x: 2., y: 3., z:2.};
    print!("Dot product : {} . {} = {}", v1, v2, dot(&v1, &v2));

    assert_eq!(Vec3f {x: 3., y: 3., z:3.}, Vec3f {x: 1., y: 2., z:3.} + Vec3f {x: 2., y: 1., z:0.});
}
