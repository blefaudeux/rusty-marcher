use geometry::Vec3f;
use shapes::Intersection;

pub fn reflect(incident: &Vec3f, normal: Vec3f) -> Vec3f {
    return *incident - normal.scaled(2. * incident.dot(&normal));
}

pub fn reflect_ray(
    incident: &Vec3f,
    intersection: &Intersection,
    refractive_index: f64,
) -> Option<(Vec3f, Vec3f)> {
    // Compute incident angle
    // See https://en.wikipedia.org/wiki/Snell%27s_law
    let mut normal = intersection.normal.clone();
    let mut c = normal.dot(incident);

    // Could be that the ray is inside the object
    let r = if c < 0. {
        refractive_index
    } else {
        1. / refractive_index
    };

    if c < 0. {
        c = -c;
        normal = -normal;
    }

    let cos_theta_2 = 1. - r * r * (1. - c * c);

    // Total refraction, no reflection
    if cos_theta_2 > 0. {
        return None;
    }

    // We got a reflection, return the ray + an origin offset from the original shape
    let reflected_ray = reflect(incident, normal);
    let reflection_orig: Vec3f;

    if reflected_ray.dot(&intersection.normal) < 0. {
        reflection_orig = intersection.point - intersection.normal.scaled(1e-4);
    } else {
        reflection_orig = intersection.point + intersection.normal.scaled(1e-4);
    };

    return Some((reflection_orig, reflected_ray));
}

pub fn refract_ray(
    incident: &Vec3f,
    intersection: &Intersection,
    refractive_index: f64,
) -> Option<(Vec3f, Vec3f)> {
    // See https://en.wikipedia.org/wiki/Snell%27s_law
    let mut normal = intersection.normal.clone();
    let mut c = -normal.dot(incident);

    // Could be that the ray is inside the object
    let r = if c < 0. {
        refractive_index
    } else {
        1. / refractive_index
    };

    if c < 0. {
        c = -c;
        normal = -normal;
    }

    let cos_theta_2 = 1. - r * r * (1. - c * c);

    // Total reflection, no refraction
    if cos_theta_2 < 0. {
        return None;
    }

    let refracted_ray =
        (incident.scaled(r) + normal.scaled(r * c - cos_theta_2.sqrt())).normalized();

    // Compute the ray origin, offset from the origin shape
    let refract_orig: Vec3f;
    if refracted_ray.dot(&normal) > 0. {
        refract_orig = intersection.point + normal.scaled(1e-4);
    } else {
        refract_orig = intersection.point - normal.scaled(1e-4);
    }

    return Some((refract_orig, refracted_ray));
}
