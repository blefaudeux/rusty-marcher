use geometry::Vec3f;
use shapes::*;

// Not implementing the Shape trait, Triangle is a basic primitive
#[derive(Clone, Debug)]
pub struct Triangle {
    pub vertices: Vec<Vec3f>,
    pub normal: Vec3f,
    pub center: Vec3f,
}

// Vertices need to be defined counter-clockwise
fn inside(a: Vec3f, p1: Vec3f, p2: Vec3f) -> bool {
    (p1 - a).cross(&(p2 - a)).z > 0.
}

impl Triangle {
    pub fn create(vertices: Vec<Vec3f>) -> Triangle {
        assert![vertices.len() == 3];

        let mean = (vertices[0] + vertices[1] + vertices[2]).scaled(1. / 3.);

        // - The plane normal is the cross product of two consecutive edges..
        let edge_1 = vertices[1] - vertices[0];
        let edge_2 = vertices[2] - vertices[1];

        Triangle {
            vertices,
            normal: edge_1.cross(&edge_2).normalized(),
            center: mean,
        }
    }

    pub fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        // Very similar to a polygon intersection,
        // but we know that we only have 3 sides here
        // Probably in need for some refactoring

        // Parallel to the plane
        let dot_product = dir.dot(&self.normal);
        if dot_product == 0. {
            return None;
        }

        // Compute the intersection point on the plane
        let dist = (self.center - *orig).dot(&self.normal) / dot_product;

        // Going away
        if dist < 0. {
            return None;
        }

        let intersect = *orig + dir.scaled(dist);

        // Does it lie within or outside of the convex polygon ?
        for i in 0..3 {
            if !inside(intersect, self.vertices[i], self.vertices[(i + 1) % 3]) {
                return None;
            }
        }

        Some(Intersection {
            point: intersect,
            normal: self.normal,
            diffuse_color: Vec3f::zero(),
        })
    }
}
