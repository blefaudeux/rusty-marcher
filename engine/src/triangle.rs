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
    (p1 - a).cross(p2 - a).z > 0.
}

#[allow(dead_code)]
impl Triangle {
    pub fn offset(&mut self, off: Vec3f) {
        self.center += off;
        for v in &mut self.vertices {
            *v += off;
        }
    }

    #[allow(dead_code)]
    pub fn scale(&mut self, s: f64) {
        for v in &mut self.vertices {
            v.scale(s);
        }
    }

    pub fn create(vertices: Vec<Vec3f>) -> Triangle {
        assert![vertices.len() == 3];

        let mean = (vertices[0] + vertices[1] + vertices[2]).scaled(1. / 3.);

        // - The plane normal is the cross product of two consecutive edges..
        let edge_1 = vertices[1] - vertices[0];
        let edge_2 = vertices[2] - vertices[1];

        Triangle {
            vertices,
            normal: edge_1.cross(edge_2).normalized(),
            center: mean,
        }
    }

    pub fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        // Very similar to a polygon intersection, but we know that we only have 3 sides here

        // Direction needs to be normalized
        assert![(dir.squared_norm() - 1.).abs() < 1e-4];

        // Parallel to the plane
        let dot_product = dir.dot(self.normal);
        if dot_product.abs() < 1e-6 {
            return None;
        }

        // Compute the intersection point on the plane
        let dist = (self.center - *orig).dot(self.normal) / dot_product;

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
            reflectance: Reflectance::create_default(),
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_intersect() {
        use super::*;
        let vertices = vec![
            Vec3f {
                x: -1.,
                y: 3.,
                z: 2.2,
            },
            Vec3f {
                x: -3.,
                y: 0.2,
                z: 2.1,
            },
            Vec3f {
                x: 0.,
                y: 1.,
                z: 2.,
            },
        ];

        let triangle1 = Triangle::create(vec![vertices[0], vertices[1], vertices[2]]);
        let triangle2 = Triangle::create(vec![vertices[1], vertices[2], vertices[0]]);
        let triangle3 = Triangle::create(vec![vertices[2], vertices[0], vertices[1]]);

        let orig = Vec3f {
            x: -1.,
            y: 2.,
            z: 5.3,
        };

        let dir = Vec3f {
            x: 0.1,
            y: -0.2,
            z: -3.,
        }
        .normalized();

        let test1 = triangle1.intersect(&orig, &dir).unwrap();
        let test2 = triangle2.intersect(&orig, &dir).unwrap();
        let test3 = triangle3.intersect(&orig, &dir).unwrap();

        assert![(test1.point - test2.point).squared_norm() < 1e-3];
        assert![(test1.normal - test2.normal).squared_norm() < 1e-3];

        assert![(test1.point - test3.point).squared_norm() < 1e-3];
        assert![(test1.normal - test3.normal).squared_norm() < 1e-3];

        assert![(triangle1.normal.squared_norm() - 1.).abs() < 1e-3];
        assert![triangle1.normal.dot(dir) < 0.];
    }
}
