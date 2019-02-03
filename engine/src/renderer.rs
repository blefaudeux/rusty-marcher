use framebuffer::FrameBuffer;
use geometry::Vec3f;
use lights::Light;
use shapes::Intersection;
use shapes::Reflectance;
use shapes::Shape;

pub struct Renderer {
    pub fov: f64,
    pub half_fov: f64,
    pub height: f64,
    pub width: f64,
    pub ratio: f64,
}

pub fn create_renderer(fov_: f64, frame: &FrameBuffer) -> Renderer {
    return Renderer {
        fov: fov_,
        half_fov: (fov_ / 2.).tan(),
        height: frame.height as f64,
        width: frame.width as f64,
        ratio: frame.width as f64 / frame.height as f64,
    };
}

impl Renderer {
    pub fn render(&self, frame: &mut FrameBuffer, shapes: Vec<&impl Shape>, lights: Vec<&Light>) {
        let mut index = 0 as usize;
        let orig = Vec3f::zero();
        let background = Vec3f {
            x: 0.1,
            y: 0.1,
            z: 0.1,
        };

        for j in 0..frame.height {
            for i in 0..frame.width {
                let dir = self.backproject(i, j);
                frame.buffer[index] = cast_ray(&orig, &dir, &shapes, &lights, &background, 1);
                index += 1;
            }
        }
    }

    fn backproject(&self, i: u32, j: u32) -> Vec3f {
        let dir = Vec3f {
            x: (2. * (i as f64 + 0.5) / self.width - 1.) * self.half_fov * self.ratio,
            y: -(2. * (j as f64 + 0.5) / self.height - 1.) * self.half_fov,
            z: -1.,
        };

        return dir.normalized();
    }
}

fn reflect_ray(incident: &Vec3f, normal: &Vec3f) -> Vec3f {
    return *incident - normal.scaled(2. * incident.dot(normal));
}

fn refract_ray(incident: &Vec3f, mut normal: Vec3f, refractive_index: f64) -> Option<Vec3f> {
    // See https://en.wikipedia.org/wiki/Snell%27s_law
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

    return Some(incident.scaled(r) + normal.scaled(r * c - cos_theta_2.sqrt()).normalized());
}

fn diffusion_factor(intersection: &Intersection, light_dir: &Vec3f) -> f64 {
    return light_dir.dot(&intersection.normal).max(0.);
}

fn specular_factor(intersection: &Intersection, origin: &Vec3f, light_dir: &Vec3f) -> f64 {
    // Compute the light reflected vector at that point
    let incident = -*light_dir;
    let reflected = reflect_ray(&incident, &intersection.normal);

    // The specular reflection coeff is the dot product in between the purely
    // reflected ray and the viewerś point of view
    let dir_to_viewer = (*origin - intersection.point).normalized();
    return reflected.dot(&dir_to_viewer).max(0.);
}

fn intersect_shapes(orig: &Vec3f, dir: &Vec3f, shapes: &Vec<&impl Shape>, shape_ref: u8) -> bool {
    let mut i = 0 as u8;
    for shape in shapes {
        if i == shape_ref {
            continue;
        }
        i += 1;

        let result = shape.intersect(orig, dir);

        match result {
            Some(_intersection) => {
                return true;
            }
            None => {
                continue;
            }
        }
    }
    return false;
}

fn direct_lighting(
    origin: &Vec3f,
    intersection: &Intersection,
    reflectance: &Reflectance,
    shapes: &Vec<&impl Shape>,
    lights: &Vec<&Light>,
    shape_ref: u8,
) -> Vec3f {
    let mut light_intensity = Vec3f::zero();

    for light in lights {
        let light_dir = (light.position - intersection.point).normalized();

        if intersect_shapes(&intersection.point, &light_dir, shapes, shape_ref) {
            // Cast shadow
            continue;
        }

        // Handle diffuse lighting
        let diffusion = diffusion_factor(&intersection, &light_dir);
        light_intensity += (light.color * reflectance.diffuse_color)
            .scaled(diffusion)
            .scaled(light.intensity);

        // Handle specular reflections
        let specular = (specular_factor(&intersection, &origin, &light_dir) * reflectance.specular)
            .powf(reflectance.specular_exponent);
        light_intensity += light.color.scaled(specular);
    }

    return light_intensity;
}

fn refracted_lighting(
    incident: &Vec3f,
    intersection: &Intersection,
    reflectance: &Reflectance,
    shapes: &Vec<&impl Shape>,
    lights: &Vec<&Light>,
    background: &Vec3f,
    n_recursion: u8,
) -> Vec3f {
    let refract = refract_ray(incident, intersection.normal, reflectance.refractive_index);

    match refract {
        Some(refracted_ray) => {
            let refract_orig: Vec3f;
            if refracted_ray.dot(&intersection.normal) > 0. {
                refract_orig = intersection.point + intersection.normal.scaled(1e-3);
            } else {
                refract_orig = intersection.point - intersection.normal.scaled(1e-3);
            }

            // TODO: Compute the refraction coeff
            return cast_ray(
                &refract_orig,
                &refracted_ray,
                shapes,
                lights,
                background,
                n_recursion + 1,
            )
            .scaled(1. - reflectance.reflection);
        }
        _ => {
            return Vec3f::zero();
        }
    }
}
fn find_closest_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    shapes: &Vec<&impl Shape>,
) -> Option<(Intersection, u8)> {
    let mut intersection_final = Intersection {
        point: Vec3f::zero(),
        normal: Vec3f::zero(),
        diffuse_color: Vec3f::zero(),
    };

    let mut hit = false;
    let mut shape_index = 0;
    let mut shape_hit = 0;
    let mut hist_closest = 0.;

    for shape in shapes {
        let test = shape.intersect(orig, dir);
        match test {
            Some(intersection) => {
                let hit_dist = (intersection.point - *orig).dot(dir);

                if !hit || hit_dist < hist_closest {
                    intersection_final = intersection;
                    hit = true;
                    shape_hit = shape_index;
                    hist_closest = hit_dist;
                }
            }
            _ => {}
        }
        shape_index += 1;
    }

    if hit {
        return Some((intersection_final, shape_hit as u8));
    }
    return None;
}

fn cast_ray(
    orig: &Vec3f,
    dir: &Vec3f,
    shapes: &Vec<&impl Shape>,
    lights: &Vec<&Light>,
    background: &Vec3f,
    n_recursion: u8,
) -> Vec3f {
    if n_recursion > 6 {
        return Vec3f::zero();
    }

    let result = find_closest_intersect(orig, dir, shapes);

    match result {
        Some(intersect_result) => {
            let intersection = &intersect_result.0;
            let i_shape = intersect_result.1;
            let shape = shapes[i_shape as usize];

            let mut light_intensity = Vec3f::zero();

            // Compute the reflections recursively
            let reflected_ray = reflect_ray(dir, &intersection.normal);
            let mut reflection_orig: Vec3f;
            if reflected_ray.dot(&intersection.normal) < 0. {
                reflection_orig = intersection.point - intersection.normal.scaled(1e-3);
            } else {
                reflection_orig = intersection.point + intersection.normal.scaled(1e-3);
            }

            light_intensity += cast_ray(
                &reflection_orig,
                &reflected_ray,
                shapes,
                lights,
                background,
                n_recursion + 1,
            )
            .scaled(shape.reflectance().reflection);

            // Go through all the lights, sum up the individual contributions
            let mut direct_light = direct_lighting(
                orig,
                &intersection,
                shape.reflectance(),
                shapes,
                lights,
                i_shape,
            );

            if n_recursion == 1 {
                direct_light.scale(shape.reflectance().diffusion);
            }

            light_intensity += direct_light;

            // Compute the refracted light
            light_intensity += refracted_lighting(
                dir,
                &intersection,
                shape.reflectance(),
                shapes,
                lights,
                background,
                n_recursion + 1,
            );

            return light_intensity;
        }
        // No intersection, do nothing and test the next shape
        _ => {
            if n_recursion == 1 {
                return *background;
            } else {
                return Vec3f::zero();
            }
        }
    }
}
