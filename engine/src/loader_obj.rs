extern crate tobj;

use std::path::Path;

pub struct Obj {
    models: Vec<tobj::Model>,
    materials: Vec<tobj::Material>,
}

pub fn load(path: String) -> Option<Obj> {
    let loaded = tobj::load_obj(&Path::new(&path));
    if !loaded.is_ok() {
        println!["Could not load obj from {}", path];
        return None;
    }

    println!["Loaded obj from {}", path];
    let (models, materials) = loaded.unwrap();
    println!["Models {}, materials {}", models.len(), materials.len()];

    return Some(Obj {
        models: models,
        materials: materials,
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_cornell_box() {
        let _test = load(String::from("../test_data/cornell_box.obj"));
    }

}
