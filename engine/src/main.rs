mod framebuffer;
mod geometry;
mod lights;
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

    // Add spheres to the scene
    // Red sphere
    reflectance.diffuse_color = Vec3f {
        x: 1.,
        y: 0.,
        z: 0.,
    };

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
    reflectance.diffuse_color = Vec3f {
        x: 0.,
        y: 0.,
        z: 1.,
    };
    let sphere_blue = sphere::create(
        Vec3f {
            x: -3.,
            y: -4.,
            z: -12.,
        },
        2.,
        reflectance,
    );

    // Green sphere
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
        x: 1.,
        y: 1.,
        z: 1.,
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

    let lights = vec![&light_white, &light_red];
    let shapes = vec![&sphere_white, &sphere_green, &sphere_blue, &sphere_red];

    ray_marcher.render(&mut frame, shapes, lights);

    // Save to file
    frame.normalize();
    frame.write_ppm("out.ppm").unwrap();
}
