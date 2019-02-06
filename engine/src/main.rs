mod framebuffer;
mod geometry;
mod lights;
mod optics;
mod renderer;
mod shapes;
mod sphere;

use geometry::Vec3f;

fn main() {
    // Allocate our dummy buffer
    let width = 1280 as u32;
    let height = 800 as u32;
    let mut frame = framebuffer::create_frame_buffer(width, height);

    let mut reflectance = shapes::create_default_reflectance();

    // TODO: Move that to a JSON loader
    // Red sphere
    reflectance.diffuse_color = Vec3f {
        x: 0.8,
        y: 0.,
        z: 0.,
    };
    reflectance.specular_exponent = 100.;

    let sphere_red = sphere::create(
        Vec3f {
            x: -5.,
            y: 0.,
            z: -16.,
        },
        2.,
        reflectance,
    );

    // Blue sphere
    reflectance.specular = 1.0;
    reflectance.diffusion = 0.2;
    reflectance.diffuse_color = Vec3f {
        x: 0.2,
        y: 0.2,
        z: 1.,
    };
    reflectance.is_glass_like = true;
    reflectance.refractive_index = 1.5;
    reflectance.reflection = 0.3;

    let sphere_blue = sphere::create(
        Vec3f {
            x: -0.5,
            y: -0.5,
            z: -5.,
        },
        2.,
        reflectance,
    );

    // Green sphere
    reflectance.diffusion = 1.;
    reflectance.reflection = 1.;
    reflectance.is_glass_like = false;
    reflectance.specular = 0.8;
    reflectance.diffuse_color = Vec3f {
        // Green sphere
        x: 0.,
        y: 1.,
        z: 0.,
    };

    let sphere_green = sphere::create(
        Vec3f {
            x: 6.,
            y: -0.5,
            z: -18.,
        },
        3.,
        reflectance,
    );

    // White sphere
    reflectance.diffuse_color = Vec3f {
        x: 0.9,
        y: 0.9,
        z: 0.9,
    };
    let sphere_white = sphere::create(
        Vec3f {
            x: 6.,
            y: 6.,
            z: -14.,
        },
        4.,
        reflectance,
    );

    // Add a light to the scene
    let light_white = lights::create_light(
        Vec3f {
            x: -20.,
            y: 20.,
            z: 20.,
        },
        Vec3f::ones(), // white light
        1.,
    );

    let light_red = lights::create_light(
        Vec3f {
            x: 20.,
            y: -20.,
            z: 20.,
        },
        Vec3f {
            x: 1.,
            y: 0.5,
            z: 0.5,
        }, // reddish light
        0.8,
    );

    // Backproject rays, save intersection status in the buffer
    let ray_marcher = renderer::create_renderer(1.7, &frame);

    let lights = vec![&light_red, &light_white];
    let shapes = vec![&sphere_green, &sphere_white, &sphere_blue, &sphere_red];

    ray_marcher.render(&mut frame, shapes, lights);

    // Save to file
    frame.normalize();
    frame.write_ppm("out.ppm").unwrap();
}
