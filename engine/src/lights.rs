use geometry::Vec3f;

pub struct Light {
    pub position: Vec3f,
    pub color: Vec3f, // RGB
    pub intensity: f64,
}

pub fn create_light(pose: Vec3f, color: Vec3f, intensity: f64) -> Light {
    return Light {
        position: pose,
        color: color.normalized_l0(),
        intensity: intensity,
    };
}
