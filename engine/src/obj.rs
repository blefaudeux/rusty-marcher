extern crate rayon;
use obj::rayon::prelude::*;

extern crate tobj;

use geometry::Vec3f;
// use polygon::*;
use shapes::*;
use std::path::Path;
use triangle::*;

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
        for t in &mut self.triangles {
            t.offset(off);
        }
    }

    pub fn update_bounding_box(&mut self) {
        let mut bb = BoundingBox::create(self.triangles[0].vertices[0]);

        for t in &mut self.triangles {
            for v in &t.vertices {
                bb.update(v);
            }
        }
        self.bounding_box = bb;
    }
}

pub fn load(path: String) -> Option<Vec<Obj>> {
    let loaded = tobj::load_obj(&Path::new(&path));
    if loaded.is_err() {
        println!["Could not load obj from {}", path];
        return None;
    }

    let (models, materials) = loaded.unwrap();

    println!["Loaded obj from {}", path];
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
                    let vertices: Vec<Vec3f> = (0..3)
                        .into_iter()
                        .map(|v| {
                            let i_v = model.mesh.indices[t * 3 + v] as usize;
                            let vertex = Vec3f {
                                x: model.mesh.positions[3 * i_v] as f64,
                                y: model.mesh.positions[3 * i_v + 1] as f64,
                                z: model.mesh.positions[3 * i_v + 2] as f64,
                            };
                            bounding_box.update(&vertex);
                            vertex
                        })
                        .collect();

                    // Return a triangle out of it
                    Triangle::create(vertices)
                })
                .collect();

            println![
                "Object bounding box: {} - {}. scale {}",
                bounding_box.min,
                bounding_box.max,
                bounding_box.scale()
            ];

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

#[allow(dead_code)]
pub fn autoscale(objects: &mut Vec<Obj>, desired_scale: f64) {
    assert_ne![desired_scale, 0.];

    // Get the scale of all objects
    let mut bb = BoundingBox::create(Vec3f::zero());

    for o in &(*objects) {
        bb.update(&o.bounding_box().min);
        bb.update(&o.bounding_box().max);
    }

    // Scale all the vertices
    if bb.scale() > 0. {
        for mut o in &mut (*objects) {
            for mut v in &mut o.triangles {
                v.offset(-bb.middle());
            }

            let s = desired_scale / bb.scale();
            for v in &mut o.triangles {
                v.scale(s);
                v.offset(bb.middle());
            }
        }
    }

    for mut o in &mut (*objects) {
        o.update_bounding_box();
        println![
            "Object bounding box: {} - {}. scale {}",
            o.bounding_box().min,
            o.bounding_box().max,
            o.bounding_box().scale()
        ];
    }
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

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box.clone()
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
