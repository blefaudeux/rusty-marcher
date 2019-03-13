extern crate rayon;
use obj::rayon::prelude::*;

extern crate tobj;

use geometry::Vec3f;
// use polygon::*;
use shapes::*;
use std::path::Path;
use triangle::*;

#[derive(Clone, Debug)]
struct BoundingBox {
    min: Vec3f,
    max: Vec3f,
}

impl BoundingBox {
    fn update(&mut self, vec: Vec3f) {
        self.min.x = f64::min(self.min.x, vec.x);
        self.min.y = f64::min(self.min.y, vec.y);
        self.min.z = f64::min(self.min.z, vec.z);

        self.max.x = f64::max(self.max.x, vec.x);
        self.max.y = f64::max(self.max.y, vec.y);
        self.max.z = f64::max(self.max.z, vec.z);
    }

    fn create(vec: Vec3f) -> BoundingBox {
        BoundingBox { min: vec, max: vec }
    }

    fn scale(&self) -> f64 {
        let diff = self.max - self.min;
        diff.max()
    }
}

#[derive(Clone, Debug)]
pub struct Obj {
    model: tobj::Model, // Model holds a mesh definition and a name
    material: Option<tobj::Material>,
    reflectances: Vec<Reflectance>,
    triangles: Vec<Triangle>,
    bounding_box: BoundingBox,
}

#[allow(dead_code)]
impl Obj {
    pub fn offset(&mut self, off: Vec3f) {
        // Only move the triangles for now, not the mesh object
        for mut t in &mut self.triangles {
            t.offset(off);
        }
    }
}
pub fn load(path: String) -> Option<Vec<Obj>> {
    let loaded = tobj::load_obj(&Path::new(&path));
    if loaded.is_err() {
        println!["Could not load obj from {}", path];
        return None;
    }

    println!["Loaded obj from {}", path];
    let (models, materials) = loaded.unwrap();
    println!["Models {}, materials {}", models.len(), materials.len()];

    // Construct independent object from the models and materials
    let objects: Vec<Obj> = models
        .into_par_iter()
        .map(|model| {
            // TODO: Handle material/reflectance properly
            let material = if let Some(id) = model.mesh.material_id {
                Some(materials[id].clone())
            } else {
                None
            };

            // Pre compute all the triangles
            let n_triangles = model.mesh.indices.len() / 3;
            println![
                "Loading {} triangles from the object : {}. {} vertices in total",
                n_triangles,
                model.name,
                model.mesh.positions.len() / 3
            ];

            // Compute the bounding box on the fly
            let mut bounding_box = BoundingBox::create(Vec3f {
                x: model.mesh.positions[0] as f64,
                y: model.mesh.positions[1] as f64,
                z: model.mesh.positions[2] as f64,
            });

            let triangles: Vec<Triangle> = (0..n_triangles)
                .into_iter()
                .map(|t| {
                    // Collect all the vertices for this face
                    let mut vertices: Vec<Vec3f> = (0..3)
                        .into_iter()
                        .map(|v| {
                            let i_v = model.mesh.indices[t * 3 + v] as usize;
                            let vertex = Vec3f {
                                x: model.mesh.positions[3 * i_v] as f64,
                                y: model.mesh.positions[3 * i_v + 1] as f64,
                                z: model.mesh.positions[3 * i_v + 2] as f64,
                            };
                            bounding_box.update(vertex);
                            vertex
                        })
                        .collect();

                    // Scale all the vertices
                    if bounding_box.scale() > 0. {
                        let s = 1. / bounding_box.scale();
                        for mut v in &mut vertices {
                            v.scale(s);
                        }
                    }

                    // Return a triangle out of it
                    Triangle::create(vertices)
                })
                .collect();

            // Get arbitrary reflectance values, continuous
            let reflectances: Vec<Reflectance> = (0..n_triangles)
                .into_iter()
                .map(|t| {
                    let mut r = Reflectance::create_default();
                    let t_f = t as f64;
                    r.diffuse_color = Vec3f {
                        x: 1. - t_f / n_triangles as f64,
                        y: t_f / n_triangles as f64,
                        z: 1.,
                    };
                    r
                })
                .collect();

            Obj {
                model,
                material,
                reflectances,
                triangles,
                bounding_box,
            }
        })
        .collect();

    Some(objects)
}

impl Shape for Obj {
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        let mut intersection_final = Intersection::create_default();

        let mut hit_triangle = false;
        let mut dist_closest = 0.;

        // Go through all triangles, return the hit closest to ray origin
        for (t_i, t) in self.triangles.iter().enumerate() {
            let res = t.intersect(orig, dir);

            if let Some(intersection) = res {
                let dist_hit = (intersection.point - *orig).squared_norm();
                if !hit_triangle || dist_hit < dist_closest {
                    // FIXME: we don't handle per-triangle color here, a bit broken
                    intersection_final = Intersection {
                        point: intersection.point,
                        normal: intersection.normal,
                        reflectance: self.reflectances[t_i],
                    };

                    hit_triangle = true;
                    dist_closest = dist_hit;
                }
            }
        }

        if hit_triangle {
            return Some(intersection_final);
        }
        None
    }
}

// Intersect with one mesh

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_cornell_box() {
        let test = load(String::from("../test_data/cornell_box.obj"));
        assert![test.is_some()];
    }

}
