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
    let mut objects = Vec::new();

    for model in models {
        let material = if let Some(id) = model.mesh.material_id {
            Some(materials[id].clone())
        } else {
            None
        };

        // Pre compute all the triangles
        let mut triangles = Vec::with_capacity(model.mesh.indices.len() / 3);

        for f in 0..model.mesh.indices.len() / 3 {
            let mut vertices = Vec::with_capacity(3);

            for i in 0..3 {
                let i_face = (model.mesh.indices[f * 3] + i) as usize;

                vertices.push(Vec3f {
                    x: model.mesh.positions[3 * i_face].into(),
                    y: model.mesh.positions[3 * i_face + 1].into(),
                    z: model.mesh.positions[3 * i_face + 2].into(),
                });
            }
            triangles.push(Triangle::create(vertices));
        }

        let mut obj = Obj {
            model,
            material,
            reflectance: Reflectance::create_default(),
            triangles,
        };

        objects.push(obj);
    }

    Some(objects)
}

impl Shape for Obj {
    fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<Intersection> {
        for t in &self.triangles {
            let res = t.intersect(orig, dir);
            if let Some(mut hit) = res {
                hit.diffuse_color = self.reflectance().diffuse_color;
                return Some(hit);
            }
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
        let test = load(String::from("../test_data/cornell_box.obj"));
        assert![test.is_some()];
    }

}
