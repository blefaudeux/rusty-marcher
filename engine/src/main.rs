mod framebuffer;
mod geometry;
mod renderer;
mod shapes;

fn main() {
    // Allocate our dummy buffer
    let width = 1280 as u32;
    let height = 800 as u32;
    let mut frame = framebuffer::create_frame_buffer(width, height);

    // Fill with a gradient
    framebuffer::fill_gradient(&mut frame);

    // Now add a sphere to the scene
    let sphere = shapes::create_sphere(
        &geometry::Vec3f {
            x: 0.,
            y: 0.,
            z: -10.,
        },
        1.,
    );

    // Backproject rays, save intersection status in the buffer
    let ray_marcher = renderer::create_renderer(0.5, &frame);
    ray_marcher.render(&mut frame, &sphere);

    // Save to file
    frame.write_ppm("out.ppm").unwrap();
}
