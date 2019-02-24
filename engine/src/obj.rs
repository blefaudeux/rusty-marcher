extern crate tobj;

use geometry::Vec3f;
use shapes::*;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Obj {
    models: Vec<tobj::Model>,
    materials: Vec<tobj::Material>,
    reflectance: Reflectance, // FIXME: Ben. Needs to be set from materials
}

pub fn _load(path: String) -> Option<Obj> {
    let loaded = tobj::load_obj(&Path::new(&path));
    if loaded.is_err() {
        println!["Could not load obj from {}", path];
        return None;
    }

    println!["Loaded obj from {}", path];
    let (models, materials) = loaded.unwrap();
    println!["Models {}, materials {}", models.len(), materials.len()];

    Some(Obj {
        models,
        materials,
        reflectance: Reflectance::create_default(), // FIXME: Ben
    })
}

impl Shape for Obj {
    fn intersect(&self, _orig: &Vec3f, _dir: &Vec3f) -> Option<Intersection> {
        // TODO: Ben
        None
    }

    fn reflectance(&self) -> &Reflectance {
        &self.reflectance
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_cornell_box() {
        let test = _load(String::from("../test_data/cornell_box.obj"));
        assert![test.is_some()];
    }

}
