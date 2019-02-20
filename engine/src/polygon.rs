use geometry::Vec3f;
use shapes::Intersection;
use shapes::Reflectance;
use shapes::Shape;

// A planar polygon
#[derive(Clone)]
pub struct ConvexPolygon {
    vertices: Vec<Vec3f>,
    reflectance: Reflectance,
    plane_normal: Vec3f,
    plane_point: Vec3f,
}

pub fn create(vertices: Vec<Vec3f>, reflectance: Reflectance) -> ConvexPolygon {
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

    return ConvexPolygon {
        vertices: vertices,
        reflectance: reflectance,
        plane_normal: -edge_1.cross(&edge_2).normalized(),
        plane_point: mean,
    };
}

// Check that the two vectors are angled by more than Pi
// ! This supposes that the polygon is defined clockwise !
// ! to be fixed in the create_polygon part !
fn inside(a: &Vec3f, p1: &Vec3f, p2: &Vec3f) -> bool {
    return (*p1 - *a).cross(&(*p2 - *a)).z < 0.;
}

// Implementing the intersect and reflectance traits
impl Shape for ConvexPolygon {
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        // Parallel to the plane
        let dotprod = dir.dot(&self.plane_normal);
        if dotprod == 0. {
            return None;
        }

        // Compute the intersection point on the plane
        let dist = (self.plane_point - *orig).dot(&self.plane_normal) / dotprod;

        // Going away
        if dist < 0. {
            return None;
        }

        let intersect = *orig + dir.scaled(dist);

        // Does it lie within or outside of the convex polygon ?
        let n_vertices = self.vertices.len();

        for i in 0..n_vertices {
            if !inside(
                &intersect,
                &self.vertices[i],
                &self.vertices[(i + 1) % n_vertices],
            ) {
                return None;
            }
        }

        return Some(Intersection {
            point: intersect,
            normal: self.plane_normal,
            diffuse_color: self.reflectance.diffuse_color,
        });;
    }

    fn reflectance(&self) -> &Reflectance {
        return &self.reflectance;
    }
}