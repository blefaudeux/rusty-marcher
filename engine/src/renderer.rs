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
use shapes::Shape;
use std::time::Instant;

pub struct Renderer {
    pub fov: f64,
    pub half_fov: f64,
    pub height: f64,
    pub width: f64,
    pub ratio: f64,
}

pub fn create_renderer(fov: f64, height: f64, width: f64) -> Renderer {
    Renderer {
        fov,
        half_fov: (fov / 2.).tan(),
        height: height,
        width: width,
        ratio: width / height,
    }
}

impl Renderer {
    pub fn render(&self, frame: &mut FrameBuffer, scene: &Scene) -> String {
        let orig = &scene.camera;
        let now = Instant::now();

        let background = Vec3f {
            x: 0.1,
            y: 0.1,
            z: 0.1,
        };

        // Distribute the computation over spatially coherent patches
        let patch_size = 32;

        if (frame.height % patch_size != 0) || (frame.width % patch_size != 0) {
            println!("Dimensions mismatch")
        }

        let n_height = frame.height / patch_size;
        let n_width = frame.width / patch_size;
        let n_patches = n_height * n_width;

        println!(
            "Rendering using patches of size {}, using {} patches overall",
            patch_size, n_patches
        );

        // Render, distribute the patches over threads
        let render_queue: Vec<Vec<Vec3f>> = (0..n_patches)
            .into_par_iter()
            .map(|p| {
                // Pre-allocate the patch
                let mut buffer: Vec<Vec3f> = Vec::with_capacity(patch_size * patch_size);

                let p_line = p % n_width * patch_size;
                let p_col = p / n_width * patch_size;
                let p_line_end = p_line + patch_size;
                let p_col_end = p_col + patch_size;

                // Backproject locally, keep spatial coherency
                for i in p_col..p_col_end {
                    for j in p_line..p_line_end {
                        buffer.push(cast_ray(
                            &orig,
                            self.backproject(j, i),
                            &scene.shapes,
                            &scene.lights,
                            &background,
                            1,
                        ));
                    }
                }
                buffer
            })
            .collect();

        // Reconstruct the picture in the framebuffer
        let mut p_width = 0;
        let mut p_height;

        for (p, render_patch) in render_queue.iter().enumerate() {
            p_height = (p / n_width) * patch_size;
            let p_height_end = p_height + patch_size;
            let p_width_end = p_width + patch_size;

            let mut k = 0;
            for j in p_height..p_height_end {
                for i in p_width..p_width_end {
                    frame.buffer[j][i] = render_patch[k];
                    k += 1;
                }
            }
            p_width = p_width_end % frame.width;
        }

        // Output some metrics
        let ms_render_time =
            now.elapsed().as_secs() * 1_000 + u64::from(now.elapsed().subsec_nanos()) / 1_000_000;
        let fps = 1000. / ms_render_time as f64;
        let pix_scale = (frame.height * frame.width) as f64 / 1e6;

        let message = format!(
            "Scene rendered in {} ms ({} fps, {:.2} MP/s)",
            ms_render_time,
            fps as u32,
            fps * pix_scale,
        );

        println!("{}", message);
        println!("{} threads used", rayon::current_num_threads());
        return message;
    }

    fn backproject(&self, i: usize, j: usize) -> Vec3f {
        Vec3f {
            x: 2. * (i as f64 / self.width - 0.5) * self.half_fov * self.ratio,
            y: -2. * (j as f64 / self.height - 0.5) * self.half_fov,
            z: -1.,
        }
        .normalized()
    }
}

fn diffusion_factor(intersection: &Intersection, light_dir: &Vec3f) -> f64 {
    light_dir.dot(intersection.normal).max(0.)
}

fn specular_factor(intersection: &Intersection, origin: &Vec3f, light_dir: &Vec3f) -> f64 {
    // Compute the light reflected vector at that point
    let incident = -*light_dir;
    let reflected = reflect(incident, intersection.normal);

    // The specular reflection coeff is the dot product in between the purely
    // reflected ray and the viewerś point of view
    let dir_to_viewer = (*origin - intersection.point).normalized();
    reflected.dot(dir_to_viewer).max(0.)
}

fn direct_lighting(
    origin: &Vec3f,
    intersection: &Intersection,
    shapes: &[Box<dyn Shape + Sync>],
    lights: &[Light],
) -> Vec3f {
    // Compute the lighting contribution of direct illumination,
    // meaning diffuse and specular lighting

    let mut light_intensity = Vec3f::zero();
    let mut intersect_orig: Vec3f;

    for light in lights {
        let light_dir = (light.position - intersection.point).normalized();

        if light_dir.dot(intersection.normal) < 0. {
            intersect_orig = intersection.point - intersection.normal.scaled(1e-3);
        } else {
            intersect_orig = intersection.point + intersection.normal.scaled(1e-3);
        }

        if intersect_shape_set(&intersect_orig, &light_dir, &shapes[..]) {
            // Cast shadow, this light is not visible from this point of view
            continue;
        }

        // Handle diffuse lighting
        let diffusion = diffusion_factor(&intersection, &light_dir);
        light_intensity += (light.color * intersection.reflectance.diffuse_color)
            .scaled(diffusion)
            .scaled(light.intensity);

        // Handle specular reflections
        let specular = (specular_factor(&intersection, &origin, &light_dir)
            * intersection.reflectance.specular)
            .powf(intersection.reflectance.specular_exponent);
        light_intensity += light.color.scaled(specular);
    }

    light_intensity.scaled(intersection.reflectance.diffusion)
}

fn reflected_lighting(
    incident: Vec3f,
    intersection: &Intersection,
    shapes: &[Box<dyn Shape + Sync>],
    lights: &[Light],
    background: &Vec3f,
    n_recursion: u8,
) -> Vec3f {
    // We may or may not have a reflected ray, angle dependent
    let reflect = reflect_ray(
        incident,
        &intersection,
        intersection.reflectance.refractive_index,
    );

    match reflect {
        Some(reflection) => cast_ray(
            &reflection.0,
            reflection.1,
            shapes,
            lights,
            background,
            n_recursion + 1,
        )
        .scaled(intersection.reflectance.reflection),
        _ => Vec3f::zero(),
    }
}

// Compute the lighting contribution of a refracted ray
fn refracted_lighting(
    incident: Vec3f,
    intersection: &Intersection,
    shapes: &[Box<dyn Shape + Sync>],
    lights: &[Light],
    background: &Vec3f,
    n_recursion: u8,
) -> Vec3f {
    // We may or may not have a reflected ray, angle dependent
    let refract = refract_ray(
        incident,
        intersection,
        intersection.reflectance.refractive_index,
    );

    match refract {
        Some(refracted_ray) => cast_ray(
            &refracted_ray.0,
            refracted_ray.1,
            shapes,
            lights,
            background,
            n_recursion + 1,
        )
        .scaled(1. - intersection.reflectance.reflection),
        _ => Vec3f::zero(),
    }
}

fn cast_ray(
    orig: &Vec3f,
    dir: Vec3f,
    shapes: &[Box<dyn Shape + Sync>],
    lights: &[Light],
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

            let mut light_intensity = *background;

            // Go through all the lights, sum up the individual contributions
            light_intensity += direct_lighting(orig, &intersection, &shapes[..], &lights[..]);

            if intersection.reflectance.is_glass_like {
                // Compute the reflections recursively
                light_intensity += reflected_lighting(
                    dir,
                    &intersection,
                    &shapes[..],
                    &lights[..],
                    background,
                    n_recursion,
                );

                // Compute the refracted light recusively
                light_intensity += refracted_lighting(
                    dir,
                    &intersection,
                    &shapes[..],
                    &lights[..],
                    background,
                    n_recursion,
                );
            }
            light_intensity
        }
        // No intersection, do nothing and test the next shape
        _ => {
            if n_recursion > 1 {
                *background
            } else {
                Vec3f::zero()
            }
        }
    }
}
