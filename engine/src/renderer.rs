use framebuffer::FrameBuffer;
use geometry::Vec3f;
use lights::Light;
use shapes::Intersection;
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

        for j in 0..frame.height {
            for i in 0..frame.width {
                let dir = self.backproject(i, j);
                frame.buffer[index] = cast_ray(dir, &shapes, &lights);
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

fn reflect(incident: &Vec3f, normal: &Vec3f) -> Vec3f {
    return *incident - normal.scaled(2. * incident.dot(normal));
}

fn diffusion_factor(intersection: &Intersection, light_dir: &Vec3f) -> f64 {
    return light_dir.dot(&intersection.normal).max(0.);
}

fn specular_factor(intersection: &Intersection, origin: &Vec3f, light_dir: &Vec3f) -> f64 {
    // Compute the light reflected vector at that point
    let incident = -*light_dir;
    let reflected = reflect(&incident, &intersection.normal);

    // The specular reflection coeff is the dot product in between the purely
    // reflected ray and the viewerś point of view
    let dir_to_viewer = (*origin - intersection.point).normalized();
    return reflected.dot(&dir_to_viewer).max(0.);
}

fn cast_ray(dir: Vec3f, shapes: &Vec<&impl Shape>, lights: &Vec<&Light>) -> Vec3f {
    let orig = Vec3f::zero();

    for shape in shapes {
        let result = shape.intersect(orig, dir);

        match result {
            Some(intersection) => {
                // We got an intersection, compute the contribution from all the lights:
                let mut light_intensity = Vec3f::zero();

                // Go through all the lights, sum up the individual contributions
                for light in lights {
                    let light_dir = (light.position - intersection.point).normalized();

                    // Handle diffuse lighting
                    let diffusion = diffusion_factor(&intersection, &light_dir);

                    light_intensity += (light.color * shape.reflectance().diffuse_color)
                        .scaled(diffusion)
                        .scaled(light.intensity);

                    // Handle specular reflections
                    let specular = (specular_factor(&intersection, &orig, &light_dir)
                        * shape.reflectance().specular)
                        .powf(shape.reflectance().specular_exponent);

                    light_intensity += light.color.scaled(specular);
                }
                return light_intensity;
            }
            // No intersection, do nothing and test the next shape
            _ => {}
        }
    }

    return Vec3f::zero(); // background color
}
