use geometry::Vec3f;
use shapes::Intersection;

pub fn reflect(incident: Vec3f, normal: Vec3f) -> Vec3f {
    incident - normal.scaled(2. * incident.dot(normal))
}

pub fn reflect_ray(
    incident: Vec3f,
    intersection: &Intersection,
    refractive_index: f64,
) -> Option<(Vec3f, Vec3f)> {
    // Compute incident angle
    // See https://en.wikipedia.org/wiki/Snell%27s_law
    let mut normal = intersection.normal;
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

    if reflected_ray.dot(intersection.normal) < 0. {
        reflection_orig = intersection.point - intersection.normal.scaled(1e-4);
    } else {
        reflection_orig = intersection.point + intersection.normal.scaled(1e-4);
    };

    Some((reflection_orig, reflected_ray))
}

pub fn refract_ray(
    incident: Vec3f,
    intersection: &Intersection,
    refractive_index: f64,
) -> Option<(Vec3f, Vec3f)> {
    // See https://en.wikipedia.org/wiki/Snell%27s_law
    let mut normal = intersection.normal;
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
    let refract_orig: Vec3f = if refracted_ray.dot(normal) > 0. {
        intersection.point + normal.scaled(1e-4)
    } else {
        intersection.point - normal.scaled(1e-4)
    };

    Some((refract_orig, refracted_ray))
}

#[cfg(test)]
mod test {
    use super::*;
    use shapes::Reflectance;

    #[test]
    fn test_reflection() {
        let incident = Vec3f {
            x: 0.5,
            y: -0.5,
            z: 0.,
        };

        let intersection = Intersection {
            point: Vec3f::zero(),
            normal: Vec3f {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            reflectance: Reflectance::create_default(),
        };

        let reference_ray = Vec3f {
            x: 0.5,
            y: 0.5,
            z: 0.,
        };

        assert_eq![reflect(incident, intersection.normal), reference_ray];

        let result = reflect_ray(incident, &intersection, 1.5);

        match result {
            Some(reflected_ray) => {
                assert_eq![reflected_ray.1, reference_ray];
            }
            None => {
                assert![false];
            }
        }
    }
}
