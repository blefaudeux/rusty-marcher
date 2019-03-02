extern crate tobj;

use geometry::Vec3f;
use shapes::*;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Obj {
    model: tobj::Model, // Model holds a mesh definition and a name
    material: Option<tobj::Material>,
    reflectance: Reflectance,
}

pub fn _load(path: String) -> Option<Vec<Obj>> {
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

        let mut obj = Obj {
            model,
            material,
            reflectance: Reflectance::create_default(),
        };

        objects.push(obj);
    }

    Some(objects)
}

impl Shape for Obj {
    fn intersect(&self, _orig: &Vec3f, _dir: &Vec3f) -> Option<Intersection> {
        // Go through all the meshes, return the nearest intersection
        // Naive way for now
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
        let test = _load(String::from("../test_data/cornell_box.obj"));
        assert![test.is_some()];
    }

}
