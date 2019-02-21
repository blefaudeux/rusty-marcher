extern crate rayon;
use renderer::rayon::prelude::*;

use framebuffer::FrameBuffer;
use geometry::Vec3f;
use lights::Light;
use optics::reflect;
use optics::reflect_ray;
use optics::refract_ray;
use scene::Scene;
use shapes::find_closest_intersect;
use shapes::intersect_shape_set;
use shapes::Intersection;
use shapes::Reflectance;
use shapes::Shape;
use std::time::Instant;

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
    pub fn render(&self, frame: &mut FrameBuffer, scene: &Scene) {
        
        let orig = Vec3f::zero();
        let now = Instant::now();

        let background = Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        // Render using line batches, distribute them over threads
        frame.buffer = (0..frame.height).into_par_iter().map(|j| {
                let mut line : Vec<Vec3f> = Vec::with_capacity(frame.width as usize);
                
                for i in 0..frame.width {
                    let dir = self.backproject(i, j);
                    line.push(cast_ray(&orig, &dir, &scene.shapes, &scene.lights, &background, 1));                
                }
                return line;
            }).collect();
        

        // Output some metrics
        let ms_render_time = now.elapsed().as_secs() * 1_000 + now.elapsed().subsec_nanos() as u64 / 1_000_0000;
        let fps = 1000. / ms_render_time as f64; 
        let pix_scale = (frame.height * frame.width) as f64 / 1e6;
        println!("Scene rendered in {} ms ({} fps, {:.2} MP/s)", ms_render_time, fps as u32, fps * pix_scale );
        println!("{} threads used",rayon::current_num_threads());
        
    }

    fn backproject(&self, i: usize, j: usize) -> Vec3f {
        let dir = Vec3f {
            x: (2. * (i as f64 + 0.5) / self.width - 1.) * self.half_fov * self.ratio,
            y: -(2. * (j as f64 + 0.5) / self.height - 1.) * self.half_fov,
            z: -1.,
        };

        return dir.normalized();
    }
}

fn diffusion_factor(intersection: &Intersection, light_dir: &Vec3f) -> f64 {
    return light_dir.dot(&intersection.normal).max(0.);
}

fn specular_factor(intersection: &Intersection, origin: &Vec3f, light_dir: &Vec3f) -> f64 {
    // Compute the light reflected vector at that point
    let incident = -*light_dir;
    let reflected = reflect(&incident, intersection.normal);

    // The specular reflection coeff is the dot product in between the purely
    // reflected ray and the viewerś point of view
    let dir_to_viewer = (*origin - intersection.point).normalized();
    return reflected.dot(&dir_to_viewer).max(0.);
}

fn direct_lighting(
    origin: &Vec3f,
    intersection: &Intersection,
    reflectance: &Reflectance,
    shapes: &Vec<Box<dyn Shape + Sync>>,
    lights: &Vec<Light>,
) -> Vec3f {
    // Compute the lighting contribution of direct illumination,
    // meaning diffuse and specular lighting

    let mut light_intensity = Vec3f::zero();
    let mut intersect_orig: Vec3f;

    for light in lights {
        let light_dir = (light.position - intersection.point).normalized();

        if light_dir.dot(&intersection.normal) < 0. {
            intersect_orig = intersection.point - intersection.normal.scaled(1e-3);
        } else {
            intersect_orig = intersection.point + intersection.normal.scaled(1e-3);
        }

        if intersect_shape_set(&intersect_orig, &light_dir, shapes) {
            // Cast shadow, this light is not visible from this point of view
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

    return light_intensity.scaled(reflectance.diffusion);
}

fn reflected_lighting(
    incident: &Vec3f,
    intersection: &Intersection,
    reflectance: &Reflectance,
    shapes: &Vec<Box<dyn Shape + Sync>>,
    lights: &Vec<Light>,
    background: &Vec3f,
    n_recursion: u8,
) -> Vec3f {
    // We may or may not have a reflected ray, angle dependent
    let reflect = reflect_ray(incident, &intersection, reflectance.refractive_index);

    match reflect {
        Some(reflection) => {
            return cast_ray(
                &reflection.0,
                &reflection.1,
                shapes,
                lights,
                background,
                n_recursion + 1,
            )
            .scaled(reflectance.reflection);
        }
        _ => {
            return Vec3f::zero();
        }
    }
}

// Compute the lighting contribution of a refracted ray
fn refracted_lighting(
    incident: &Vec3f,
    intersection: &Intersection,
    reflectance: &Reflectance,
    shapes: &Vec<Box<dyn Shape + Sync>>,
    lights: &Vec<Light>,
    background: &Vec3f,
    n_recursion: u8,
) -> Vec3f {
    // We may or may not have a reflected ray, angle dependent
    let refract = refract_ray(incident, intersection, reflectance.refractive_index);

    match refract {
        Some(refracted_ray) => {
            return cast_ray(
                &refracted_ray.0,
                &refracted_ray.1,
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

fn cast_ray(
    orig: &Vec3f,
    dir: &Vec3f,
    shapes: &Vec<Box<dyn Shape + Sync>>,
    lights: &Vec<Light>,
    background: &Vec3f,
    n_recursion: u8,
) -> Vec3f {
    if n_recursion > 3 {
        return *background;
    }

    let result = find_closest_intersect(orig, dir, shapes);

    match result {
        Some(intersect_result) => {
            let intersection = &intersect_result.0;
            let shape_hit = &shapes[intersect_result.1 as usize];

            let mut light_intensity = *background;

            // Go through all the lights, sum up the individual contributions
            light_intensity +=
                direct_lighting(orig, &intersection, shape_hit.reflectance(), shapes, lights);

            if shape_hit.reflectance().is_glass_like {
                // Compute the reflections recursively
                light_intensity += reflected_lighting(
                    dir,
                    &intersection,
                    shape_hit.reflectance(),
                    shapes,
                    lights,
                    background,
                    n_recursion,
                );

                // Compute the refracted light recusively
                light_intensity += refracted_lighting(
                    dir,
                    &intersection,
                    shape_hit.reflectance(),
                    shapes,
                    lights,
                    background,
                    n_recursion,
                );
            }
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
