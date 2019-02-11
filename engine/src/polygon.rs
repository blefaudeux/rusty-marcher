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
        plane_normal: edge_1.cross(&edge_2).normalized(),
        plane_point: mean,
    };
}

fn left_of(a: &Vec3f, b: &Vec3f, c: &Vec3f) -> bool {
    return (*a - *b).dot(&(*a - *c)) > 0.;
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

        // Does it lie within or outside of the polygon ?
        // Easy subcase: our particular polygon is convex, so we just need to test the cross products
        let mut prev = left_of(&intersect, &self.vertices[0], &self.vertices[1]);
        let n_vertices = self.vertices.len();

        for i in 1..n_vertices {
            let next = left_of(
                &intersect,
                &self.vertices[i],
                &self.vertices[(i + 1) % n_vertices],
            );
            if prev ^ next {
                return None;
            }
            prev = next;
        }

        return Some(Intersection {
            point: intersect,
            normal: self.plane_normal,
            diffuse_color: self.reflectance.diffuse_color,
        });
    }

    fn reflectance(&self) -> &Reflectance {
        return &self.reflectance;
    }
}
