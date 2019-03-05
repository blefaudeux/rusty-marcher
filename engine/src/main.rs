mod framebuffer;
mod geometry;
mod lights;
mod obj;
mod optics;
mod polygon;
mod renderer;
mod scene;
mod shapes;
mod sphere;
mod triangle;

fn main() {
    // Allocate our dummy buffer
    let width = 800;
    let height = 600;
    let mut frame = framebuffer::create_frame_buffer(width, height);

    // Create renderer and scene
    let ray_marcher = renderer::create_renderer(1.5, &frame);
    let mut scene = scene::Scene::create_default();

    // Cornell Box on top
    // Load the default cornell box / obj
    let payload = obj::load(String::from("../test_data/cornell_box.obj"));

    if let Some(objects) = payload {
        for mut obj in objects {
            obj.offset(0., 0., -15.);
            scene.shapes.push(Box::new(obj));
        }
    }

    // Backproject rays, save intersection status in the buffer
    ray_marcher.render(&mut frame, &scene);

    // Save to file
    frame.normalize();
    frame.write_ppm("out.ppm").unwrap();
}
