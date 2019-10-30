#[macro_use]
extern crate relm;

#[macro_use]
extern crate relm_derive;

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

// use std::env;

// fn help() {
//     println!["Usage:"];
//     println!["renderer (path to obj) (width) (height)"];
//     println!["Using default settings\n"];
// }

// fn not_main() {
//     // Allocate our dummy buffer
//     let mut width = 640;
//     let mut height = 480;
//     let mut filepath = String::from(String::from("../test_data/cornell_box.obj"));

//     // Handle command-line arguments
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 4 {
//         help();
//     } else {
//         filepath = args[1].clone();
//         println!["Filepath: {}", filepath];

//         width = match args[2].parse() {
//             Ok(n) => n,
//             Err(_) => {
//                 eprintln!("error: second argument not an integer");
//                 help();
//                 return;
//             }
//         };

//         height = match args[3].parse() {
//             Ok(n) => n,
//             Err(_) => {
//                 eprintln!("error: second argument not an integer");
//                 help();
//                 return;
//             }
//         };
//     }

//     let mut frame = framebuffer::create_frame_buffer(width, height);

//     // Create renderer and scene
//     let ray_marcher = renderer::create_renderer(1.5, &frame);
//     let mut scene = scene::Scene::create_default();
//     scene.shapes.clear(); // Just use the obj we loaded

//     let payload = obj::load(filepath);

//     if let Some(mut objects) = payload {
//         obj::autoscale(&mut objects, 10.);

//         let off = geometry::Vec3f {
//             x: 0.,
//             y: 0.,
//             z: -500.,
//         };

//         for mut obj in objects {
//             obj.offset(off);
//             scene.shapes.push(Box::new(obj));
//         }
//     }

//     // Back-project rays, save intersection status in the buffer
//     ray_marcher.render(&mut frame, &scene);

//     // Save to file
//     frame.normalize();
//     frame.write_ppm("out.ppm").unwrap();
// }

extern crate futures_glib;
extern crate gdk_pixbuf;
extern crate gtk;

use futures_glib::Timeout;
use gdk_pixbuf::{Pixbuf, PixbufLoader};
use gtk::Orientation::Vertical;
use gtk::{
    Button, ButtonExt, ContainerExt, Image, ImageExt, Inhibit, Label, LabelExt, WidgetExt, Window,
    WindowType,
};
use relm::{Relm, Update, Widget};
use std::time::Duration;

struct Model {
    relm: Relm<Win>,
    started_rendering: Option<renderer::Renderer>,
}

#[derive(Msg)]
enum Msg {
    ToggleRendering,
    Quit,
    UpdateRaytraceImage(()),
}

fn framebuffer_to_pixbuf(fb: &framebuffer::FrameBuffer) -> Pixbuf {
    // Transform the FrameBuffer into a vector of u8
    let loader = PixbufLoader::new();
    loader.loader_write(&fb.to_vec()[..]).unwrap();
    loader.close().unwrap();

    // Return the GTK compliant Pixbuf
    loader.get_pixbuf().unwrap()
}

// Create the structure that holds the widgets used in the view.
struct Win {
    state_label: Label,
    image: Image,
    model: Model,
    window: Window,
    fb: framebuffer::FrameBuffer,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            started_rendering: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::ToggleRendering => {
                if self.model.started_rendering.is_some() {
                    self.clear_renderer();
                } else {
                    self.new_renderer();
                    self.set_msg_timeout(10, Msg::UpdateRaytraceImage);
                }
            }
            Msg::UpdateRaytraceImage(()) => {
                if self.model.started_rendering.is_some() {
                    self.update_raytrace_image();
                    self.set_msg_timeout(10, Msg::UpdateRaytraceImage);
                }
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(Vertical, 0);

        let state_label = Label::new("wait to toggle camera");
        vbox.add(&state_label);

        let toggle_camera_button = Button::new_with_label("toggle camera");
        vbox.add(&toggle_camera_button);

        let image = Image::new();
        vbox.add(&image);

        let window = Window::new(WindowType::Toplevel);
        window.add(&vbox);
        window.show_all();

        // Create the raytrace framebuffer
        let fb = framebuffer::create_frame_buffer(800, 600);

        // Send the message Increment when the button is clicked.
        connect!(
            relm,
            toggle_camera_button,
            connect_clicked(_),
            Msg::ToggleRendering
        );
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            state_label: state_label,
            image: image,
            model,
            window: window,
            fb: fb,
        }
    }
}

impl Win {
    fn set_msg_timeout<CALLBACK>(&mut self, millis: u64, callback: CALLBACK)
    where
        CALLBACK: Fn(()) -> Msg + 'static,
    {
        let stream = Timeout::new(Duration::from_millis(millis));
        self.model.relm.connect_exec_ignore_err(stream, callback);
    }

    fn update_raytrace_image(&mut self) {
        // Kick a new rendering
        if self.model.started_rendering.is_some() {
            let renderer = self.model.started_rendering.as_mut().unwrap();
            renderer.render(&mut self.fb, &scene::Scene::create_default());
        }

        // Copy back the result
        let image = &self.image;
        let pixbuf = framebuffer_to_pixbuf(&self.fb);
        image.set_from_pixbuf(&pixbuf);
        while gtk::events_pending() {
            gtk::main_iteration_do(true);
        }
    }

    fn new_renderer(&mut self) {
        // Create the renderer
        // FIXME: the fb is only use for sizing purposes, should be cleaned
        let ray_marcher = renderer::create_renderer(1.5, &self.fb);
        self.model.started_rendering = Some(ray_marcher);
        self.state_label.set_text("Created new ray tracing engine");
    }

    fn clear_renderer(&mut self) {
        self.model.started_rendering = None;
        self.state_label.set_text("Cleared renderer");
    }
}

fn main() {
    Win::run(()).unwrap();
}
