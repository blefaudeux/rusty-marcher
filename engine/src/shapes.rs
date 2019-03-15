use geometry::Vec3f;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub reflectance: Reflectance,
}

impl Intersection {
    pub fn create_default() -> Intersection {
        Intersection {
            point: Vec3f::zero(),
            normal: Vec3f::zero(),
            reflectance: Reflectance::create_default(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Reflectance {
    // Direct lighting
    pub diffusion: f64,
    pub diffuse_color: Vec3f,   // Lambertian
    pub specular: f64,          // "hard" reflectance
    pub specular_exponent: f64, // More or less mirror-like

    // Reflection / refraction
    pub is_glass_like: bool,
    pub reflection: f64,
    pub refractive_index: f64,
}

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Vec3f,
    pub max: Vec3f,
}

pub trait Shape {
    // A Shape is able to report an hypothetical intersection.
    // if true the intersect point, normal, and diffuse color
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection>;

    // Useful for fast intersect test
    fn bounding_box(&self) -> BoundingBox;
}

impl Reflectance {
    pub fn create_default() -> Reflectance {
        Reflectance {
            diffusion: 1.,
            diffuse_color: Vec3f::ones(),
            specular: 1.,
            specular_exponent: 30.,
            is_glass_like: false,
            reflection: 0.95,
            refractive_index: 1., // TODO: indices over R,G,B
        }
    }
}

impl BoundingBox {
    pub fn update(&mut self, vec: Vec3f) {
        self.min.x = self.min.x.min(vec.x);
        self.min.y = self.min.y.min(vec.y);
        self.min.z = self.min.z.min(vec.z);

        self.max.x = self.max.x.max(vec.x);
        self.max.y = self.max.y.max(vec.y);
        self.max.z = self.max.z.max(vec.z);
    }

    pub fn create(vec: Vec3f) -> BoundingBox {
        BoundingBox { min: vec, max: vec }
    }

    pub fn scale(&self) -> f64 {
        let diff = (self.max - self.min).abs();
        diff.max()
    }

    pub fn middle(&self) -> Vec3f {
        (self.max + self.min).scaled(0.5)
    }
}

// ************************************************************
// Some generic functions dealing with a collection of Shapes
// ************************************************************

pub fn intersect_shape_set(orig: &Vec3f, dir: &Vec3f, shapes: &[Box<dyn Shape + Sync>]) -> bool {
    // Check wether a ray intersects with *any* shape, in no particular order
    // Useful to compute cast shadows
    for shape in shapes {
        let result = shape.intersect(orig, dir);

        match result {
            Some(_intersection) => {
                return true;
            }
            None => {
                continue;
            }
        }
    }
    false
}

pub fn find_closest_intersect(
    orig: &Vec3f,
    dir: Vec3f,
    shapes: &[Box<dyn Shape + Sync>],
) -> Option<(Intersection, u8)> {
    // Intersect a ray with all the provided shapes,
    // return either the intersection the closest to the ray origin,
    // or nothing

    let mut intersection_final = Intersection::create_default();

    let mut hit = false;
    let mut shape_hit = 0;
    let mut dist_closest = 0.;

    for (shape_index, shape) in shapes.iter().enumerate() {
        let test = shape.intersect(orig, &dir);
        if let Some(intersection) = test {
            let dist_hit = (intersection.point - *orig).squared_norm();

            if !hit || dist_hit < dist_closest {
                intersection_final = intersection;
                hit = true;
                shape_hit = shape_index;
                dist_closest = dist_hit;
            }
        }
    }

    if hit {
        return Some((intersection_final, shape_hit as u8));
    }
    None
}
