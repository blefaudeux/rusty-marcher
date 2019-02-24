use geometry::Vec3f;

#[derive(Debug)]
pub struct Light {
    pub position: Vec3f,
    pub color: Vec3f, // RGB
    pub intensity: f64,
}

pub fn create_light(position: Vec3f, color: Vec3f, intensity: f64) -> Light {
    Light {
        position,
        color: color.normalized_l0(),
        intensity,
    }
}
