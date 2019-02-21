mod framebuffer;
mod geometry;
mod lights;
mod optics;
mod polygon;
mod renderer;
mod scene;
mod shapes;
mod sphere;

fn main() {
    // Allocate our dummy buffer
    let width = 3000;
    let height = 3000;
    let mut frame = framebuffer::create_frame_buffer(width, height);

    // Create renderer and scene
    let ray_marcher = renderer::create_renderer(1.7, &frame);
    let scene = scene::create_default();

    // Backproject rays, save intersection status in the buffer
    ray_marcher.render(&mut frame, &scene);

    // Save to file
    frame.normalize();
    frame.write_ppm("out.ppm").unwrap();
}
