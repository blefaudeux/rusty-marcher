use geometry;
use geometry::Vec3f;

pub trait Shape {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> (bool, f64);
}

// Out most basic shape: a simple sphere, easy to intersect
pub struct Sphere {
    pub center: Vec3f,
    pub radius_square: f64,
}

pub fn create_sphere(c: &Vec3f, radius: f64) -> Sphere {
    return Sphere {
        center: c.clone(),
        radius_square: radius * radius,
    };
}

// Sphere implements the Shape trait, you can intersect it
impl Shape for Sphere {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> (bool, f64) {
        let line = self.center.clone() - orig;

        let tca = geometry::dot(&line, &dir);

        let d2 = geometry::dot(&line, &line) - tca * tca;
        if d2 > self.radius_square {
            return (false, 0.);
        }

        let thc = (self.radius_square - d2).sqrt();

        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0. {
            t0 = t1;
        }

        if t0 < 0. {
            return (false, t0);
        }

        return (true, t0);
    }
}
