mod framebuffer;
mod geometry;
mod lights;
mod renderer;
mod shapes;

use geometry::Vec3f;

fn main() {
    // Allocate our dummy buffer
    let width = 1280 as u32;
    let height = 800 as u32;
    let mut frame = framebuffer::create_frame_buffer(width, height);

    // Fill with a gradient
    framebuffer::fill_gradient(&mut frame);

    // Add a sphere to the scene
    let sphere = shapes::create_sphere(
        Vec3f {
            x: 0.,
            y: 0.,
            z: -10.,
        },
        1.,
        Vec3f {
            x: 1.,
            y: 1.,
            z: 1.,
        },
    );

    // Add a light to the scene
    let light = lights::create_light(
        Vec3f {
            x: 3.,
            y: 3.,
            z: 0.,
        },
        Vec3f {
            x: 1.,
            y: 1.,
            z: 1.,
        },
    );

    // Backproject rays, save intersection status in the buffer
    let ray_marcher = renderer::create_renderer(0.5, &frame);

    let lights = vec![&light];
    let shapes = vec![&sphere];

    ray_marcher.render(&mut frame, shapes, lights);

    // Save to file
    frame.write_ppm("out.ppm").unwrap();
}
