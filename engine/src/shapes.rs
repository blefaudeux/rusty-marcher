use geometry::Vec3f;

pub struct Intersection {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub diffuse_color: Vec3f,
}

pub trait Shape: Copy {
    // A Shape is able to report an hypothetical intersection.
    // if true the intersect point, normal, and diffuse color
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<Intersection>;

    // A Shape exhibits a
    fn diffuse_color(&self) -> Vec3f;
}

// Our most basic shape: a simple sphere, easy to intersect
#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3f,
    radius_square: f64,
    diffuse_color: Vec3f,
}

pub fn create_sphere(center: Vec3f, radius: f64, diffuse_color: Vec3f) -> Sphere {
    return Sphere {
        center: center,
        radius_square: radius * radius,
        diffuse_color: diffuse_color.normalized_l0(),
    };
}

// Sphere implements the Shape trait, you can intersect it
impl Shape for Sphere {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<Intersection> {
        let line = self.center.clone() - orig;

        let tca = line.dot(&dir);
        let d2 = line.dot(&line) - tca * tca;

        if d2 > self.radius_square {
            return None;
        }

        let thc = (self.radius_square - d2).sqrt();

        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0. {
            t0 = t1;
        }

        if t0 < 0. {
            return None;
        }

        // We've had an intersection
        let intersection_point = orig + dir.scaled(t0);

        return Some(Intersection {
            point: intersection_point,
            normal: (intersection_point - self.center).normalized(),
            diffuse_color: self.diffuse_color,
        });
    }

    fn diffuse_color(&self) -> Vec3f {
        return self.diffuse_color;
    }
}
