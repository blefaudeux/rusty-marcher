use geometry::Vec3f;
use shapes::*;

// Our most basic shape: a simple sphere, easy to intersect
#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3f,
    radius_square: f64,
    reflectance: Reflectance,
    bounding_box: BoundingBox,
}

pub fn create(center: Vec3f, radius: f64, reflectance: Reflectance) -> Sphere {
    Sphere {
        center,
        radius_square: radius * radius,
        reflectance,
        bounding_box: BoundingBox {
            min: center - Vec3f::ones().scaled(radius),
            max: center + Vec3f::ones().scaled(radius),
        },
    }
}

// Sphere implements the Shape trait, you can intersect it
impl Shape for Sphere {
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        let line = self.center - *orig;

        // Direction needs to be normalized
        assert![(dir.squared_norm() - 1.).abs() < 1e-4];

        let tca = line.dot(*dir);
        let d2 = line.dot(line) - tca * tca;

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

        // We've an intersection
        let intersection_point = *orig + dir.scaled(t0);

        Some(Intersection {
            point: intersection_point,
            normal: (intersection_point - self.center).normalized(),
            reflectance: self.reflectance,
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box.clone()
    }
}
