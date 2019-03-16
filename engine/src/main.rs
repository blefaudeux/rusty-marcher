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

use std::env;

fn main() {
    // Allocate our dummy buffer
    let mut width = 640;
    let mut height = 480;
    let mut filepath = String::from(String::from("../test_data/dodecahedron.obj"));

    // Handle command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!["Usage:"];
        println!["renderer (path to obj) (width) (height)"];
        println!["Using default settings\n"];
    } else {
        filepath = args[0].clone();

        // Ok to panic if not parsable, nothing to do
        width = args[1].parse::<usize>().unwrap();
        height = args[2].parse::<usize>().unwrap();
    }

    let mut frame = framebuffer::create_frame_buffer(width, height);

    // Create renderer and scene
    let ray_marcher = renderer::create_renderer(1.5, &frame);
    let mut scene = scene::Scene::create_default();
    // scene.shapes.clear(); // Just use the obj we loaded

    let payload = obj::load(filepath);

    if let Some(mut objects) = payload {
        obj::autoscale(&mut objects, 10.);

        let off = geometry::Vec3f {
            x: 0.,
            y: 5.,
            z: -100.,
        };

        for mut obj in objects {
            obj.offset(off);
            scene.shapes.push(Box::new(obj));
        }
    }

    // Back-project rays, save intersection status in the buffer
    ray_marcher.render(&mut frame, &scene);

    // Save to file
    frame.normalize();
    frame.write_ppm("out.ppm").unwrap();
}
