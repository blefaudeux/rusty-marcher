use geometry::Vec3f;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub diffuse_color: Vec3f,
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

pub trait Shape {
    // A Shape is able to report an hypothetical intersection.
    // if true the intersect point, normal, and diffuse color
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection>;

    // A Shape exhibits a given behaviour with respect to lighting
    fn reflectance(&self) -> &Reflectance;
}

impl Reflectance {
    pub fn create_default() -> Reflectance {
        return Reflectance {
            diffusion: 1.,
            diffuse_color: Vec3f::ones(),
            specular: 1.,
            specular_exponent: 30.,
            is_glass_like: false,
            reflection: 0.95,
            refractive_index: 1., // TODO: indices over R,G,B
        };
    }
}

// ************************************************************
// Some generic functions dealing with a collection of Shapes
// ************************************************************

pub fn intersect_shape_set(orig: &Vec3f, dir: &Vec3f, shapes: &Vec<Box<dyn Shape + Sync>>) -> bool {
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
    return false;
}

pub fn find_closest_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    shapes: &Vec<Box<dyn Shape + Sync>>,
) -> Option<(Intersection, u8)> {
    // Intersect a ray with all the provided shapes,
    // return either the intersection the closest to the ray origin,
    // or nothing

    let mut intersection_final = Intersection {
        point: Vec3f::zero(),
        normal: Vec3f::zero(),
        diffuse_color: Vec3f::zero(),
    };

    let mut hit = false;
    let mut shape_index = 0;
    let mut shape_hit = 0;
    let mut dist_closest = 0.;

    for shape in shapes {
        let test = shape.intersect(orig, dir);
        match test {
            Some(intersection) => {
                let dist_hit = (intersection.point - *orig).squared_norm();

                if !hit || dist_hit < dist_closest {
                    intersection_final = intersection;
                    hit = true;
                    shape_hit = shape_index;
                    dist_closest = dist_hit;
                }
            }
            _ => {}
        }
        shape_index += 1;
    }

    if hit {
        return Some((intersection_final, shape_hit as u8));
    }
    return None;
}
