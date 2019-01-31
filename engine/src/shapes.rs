use geometry;
use geometry::Vec3f;

pub trait Shape: Copy {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> (bool, f64);
}

// Out most basic shape: a simple sphere, easy to intersect
#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius_square: f64,
    pub diffuse_color: Vec3f,
}

pub fn create_sphere(center_: Vec3f, radius: f64, diffuse_color_: Vec3f) -> Sphere {
    return Sphere {
        center: center_,
        radius_square: radius * radius,
        diffuse_color: diffuse_color_,
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
