extern crate tobj;

use geometry::Vec3f;
use shapes::*;
use std::path::Path;
use triangle::*;

#[derive(Clone, Debug)]
pub struct Obj {
    model: tobj::Model, // Model holds a mesh definition and a name
    material: Option<tobj::Material>,
    reflectance: Reflectance,
    triangles: Vec<Triangle>,
}

impl Obj {
    pub fn offset(&mut self, x: f64, y: f64, z: f64) {
        // Only move the triangles for now, not the mesh object
        let off = Vec3f { x, y, z };

        for mut t in &mut self.triangles {
            t.center += off;
            for mut v in &mut t.vertices {
                *v += off;
            }
        }
    }
}
pub fn load(path: String, scale: f64) -> Option<Vec<Obj>> {
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
        .into_iter()
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
                "Loading {} triangles from the object : {}",
                n_triangles, model.name
            ];

            let triangles: Vec<Triangle> = (0..n_triangles)
                .into_iter()
                .map(|f| {
                    // Collect all the vertices for this face
                    let vertices: Vec<Vec3f> = (0..3)
                        .into_iter()
                        .map(|i| {
                            let i_v = model.mesh.indices[f * 3 + i] as usize;
                            Vec3f {
                                x: scale * model.mesh.positions[3 * i_v] as f64,
                                y: scale * model.mesh.positions[3 * i_v + 1] as f64,
                                z: scale * model.mesh.positions[3 * i_v + 2] as f64,
                            }
                        })
                        .collect();

                    // Return a triangle out of it
                    Triangle::create(vertices)
                })
                .collect();

            Obj {
                model,
                material,
                reflectance: Reflectance::create_default(),
                triangles,
            }
        })
        .collect();

    Some(objects)
}

impl Shape for Obj {
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        let mut hit_triangle = false;
        let mut dist_closest = 0.;

        let mut intersection_final = Intersection {
            point: Vec3f::zero(),
            normal: Vec3f::zero(),
            diffuse_color: self.reflectance().diffuse_color,
        };

        // Go through all triangles, return the hit closest to ray origin
        for t in &self.triangles {
            let res = t.intersect(orig, dir);

            if let Some(intersection) = res {
                let dist_hit = (intersection.point - *orig).squared_norm();
                if !hit_triangle || dist_hit < dist_closest {
                    intersection_final = intersection;
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

    fn reflectance(&self) -> &Reflectance {
        &self.reflectance
    }
}

// Intersect with one mesh

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_cornell_box() {
        let test = load(String::from("../test_data/cornell_box.obj"), 1.);
        assert![test.is_some()];
    }

}
