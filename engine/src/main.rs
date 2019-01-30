mod frame_to_disk;
mod framebuffer;
mod geometry;
mod shapes;

fn main() {
    // Allocate our dummy buffer
    let width = 1280 as u32;
    let height = 800 as u32;
    let mut frame = framebuffer::create_frame_buffer(width, height);

    // DEBUG: Fill with a gradient
    framebuffer::fill_gradient(&mut frame);

    // Now add a sphere to the scene
    let _ = shapes::buildSphere(
        &geometry::Vec3f {
            x: 0.,
            y: 0.,
            z: 10.,
        },
        1.,
    );

    // DEBUG
    // Backproject rays, save intersection status in the buffer
    // TODO: Ben

    // Save to file
    frame_to_disk::write_ppm("out.ppm", &frame.buffer, &width, &height).unwrap();
}
