use geometry::Vec3f;
use shapes::*;

// A planar polygon
#[derive(Clone, Debug)]
pub struct ConvexPolygon {
    vertices: Vec<Vec3f>,
    reflectance: Reflectance,
    plane_normal: Vec3f,
    plane_point: Vec3f,
    bounding_box: BoundingBox,
}

#[allow(dead_code)]
impl ConvexPolygon {
    pub fn create(vertices: Vec<Vec3f>, reflectance: Reflectance) -> ConvexPolygon {
        // We want triangles, at minima
        assert![vertices.len() > 2];

        // Pre-compute the plane coefficients
        // - Compute the center of the polygon
        let mut mean = Vec3f::zero();
        let mut bounding_box = BoundingBox::create(vertices[0]);

        for v in &vertices {
            mean += *v;
            bounding_box.update(*v);
        }
        mean.scale(1. / vertices.len() as f64);

        // - The plane normal is the cross product of two consecutive edges..
        let edge_1 = vertices[1] - vertices[0];
        let edge_2 = vertices[2] - vertices[1];

        ConvexPolygon {
            vertices,
            reflectance,
            plane_normal: edge_1.cross(edge_2).normalized(),
            plane_point: mean,
            bounding_box,
        }
    }

    pub fn offset(&mut self, off: Vec3f) {
        self.plane_point += off;
        for mut v in &mut self.vertices {
            *v += off;
        }
    }
}

// Check that the two vectors are angled by less than Pi
// ! This supposes that the polygon is defined counter-clockwise !
fn inside(a: Vec3f, p1: Vec3f, p2: Vec3f) -> bool {
    (p1 - a).cross(p2 - a).z > 0.
}

// Implementing the intersect and reflectance traits
impl Shape for ConvexPolygon {
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        // Direction needs to be normalized
        assert![(dir.squared_norm() - 1.).abs() < 1e-4];

        // Parallel to the plane
        let dotprod = dir.dot(self.plane_normal);
        if dotprod == 0. {
            return None;
        }

        // Compute the intersection point on the plane
        let dist = (self.plane_point - *orig).dot(self.plane_normal) / dotprod;

        // Going away
        if dist < 0. {
            return None;
        }

        let intersect = *orig + dir.scaled(dist);

        // Does it lie within or outside of the convex polygon ?
        let n_vertices = self.vertices.len();

        for i in 0..n_vertices {
            if !inside(
                intersect,
                self.vertices[i],
                self.vertices[(i + 1) % n_vertices],
            ) {
                return None;
            }
        }

        Some(Intersection {
            point: intersect,
            normal: self.plane_normal,
            reflectance: self.reflectance,
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box.clone()
    }
}
