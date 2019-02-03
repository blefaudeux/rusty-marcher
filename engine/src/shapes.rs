use geometry::Vec3f;

pub struct Intersection {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub diffuse_color: Vec3f,
}

#[derive(Copy, Clone)]
pub struct Reflectance {
    pub ambient_color: Vec3f,
    pub diffusion: f64,
    pub diffuse_color: Vec3f,   // Lambertian
    pub specular: f64,          // "hard" reflectance
    pub specular_exponent: f64, // More or less mirror-like
    pub reflection: f64,
    pub refractive_index: f64,
}

pub fn create_default_reflectance() -> Reflectance {
    return Reflectance {
        ambient_color: Vec3f::ones(),
        diffusion: 1.,
        diffuse_color: Vec3f::ones(),
        specular: 1.,
        specular_exponent: 30.,
        reflection: 0.95,
        refractive_index: 1., // TODO: indices over R,G,B
    };
}

pub trait Shape: Copy {
    // A Shape is able to report an hypothetical intersection.
    // if true the intersect point, normal, and diffuse color
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection>;

    // A Shape exhibits a given behaviour with respect to lighting
    fn reflectance(&self) -> &Reflectance;
}
