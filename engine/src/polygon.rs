use geometry::Vec3f;
use shapes::Intersection;
use shapes::Reflectance;
use shapes::Shape;

// Our most basic shape: a simple sphere, easy to intersect
#[derive(Clone)]
pub struct Polygon {
    vertices: Vec<Vec3f>,
    reflectance: Reflectance,
    plane_normal: Vec3f,
    plane_point: Vec3f,
}

pub fn create(vertices: Vec<Vec3f>, reflectance: Reflectance) -> Polygon {
    // We want triangles, at minima
    assert![vertices.len() > 2];

    // Pre-compute the plane coefficients
    // - Compute the center of the polygon
    let mut mean = Vec3f::zero();
    for v in &vertices {
        mean += *v;
    }
    mean.scale(1. / vertices.len() as f64);

    // - The plane normal is the cross product of two consecutive edges..
    let edge_1 = vertices[1] - vertices[0];
    let edge_2 = vertices[2] - vertices[1];

    return Polygon {
        vertices: vertices,
        reflectance: reflectance,
        plane_normal: edge_1.cross(&edge_2).normalized(),
        plane_point: mean,
    };
}

// Sphere implements the Shape trait, you can intersect it
impl Shape for Polygon {
    fn intersect(&self, _orig: &Vec3f, _dir: &Vec3f) -> Option<Intersection> {
        return None;
    }

    fn reflectance(&self) -> &Reflectance {
        return &self.reflectance;
    }
}
