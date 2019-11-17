#[macro_use]
extern crate relm;

#[macro_use]
extern crate relm_derive;

extern crate gdk_pixbuf;
extern crate gtk;

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

use gdk_pixbuf::Pixbuf;
use gtk::*;
use relm::{Relm, Update, Widget};

struct Model {
    relm: Relm<Win>,
    started_rendering: Option<renderer::Renderer>,
}

#[derive(Msg)]
enum Msg {
    ToggleRendering,
    Quit,
    ToggleSaveToFile,
    ToggleOpenFile,
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
                    self.update_raytrace_image(&scene::Scene::create_default());
                }
            }
            Msg::ToggleSaveToFile => {
                self.save_to_file();
            }
            Msg::Quit => gtk::main_quit(),
            Msg::ToggleOpenFile => {
                // Open file dialog
                let open_file_dialog = FileChooserDialog::with_buttons(
                    Some("Open .obj file"),
                    Some(&self.window),
                    FileChooserAction::Open,
                    &[
                        ("_Cancel", ResponseType::Cancel),
                        ("_Open", ResponseType::Accept),
                    ],
                );

                // Filter: we just want .obj files
                let filter = FileFilter::new();
                filter.add_pattern("*.obj");
                open_file_dialog.set_filter(&filter);

                let button_pressed = open_file_dialog.run();
                let path = open_file_dialog.get_filename();
                open_file_dialog.destroy();

                match path {
                    Some(filepath) => {
                        println!("Got {:?} from FileChooserDialog", filepath);
                        if button_pressed == ResponseType::Accept.into() {
                            self.open_obj(filepath);
                        }
                    }
                    None => {}
                }
            }
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
        let vbox = gtk::Box::new(Orientation::Vertical, 0);
        let state_label = Label::new(Some("Waiting to create the renderer"));
        vbox.add(&state_label);

        // - toolbar
        let hbox = gtk::Box::new(Orientation::Horizontal, 0);

        let toggle_save_to_file = Button::new_with_label("Save to file");
        hbox.add(&toggle_save_to_file);

        let toggle_render = Button::new_with_label("Render !");
        hbox.add(&toggle_render);

        let toggle_open_file = Button::new_with_label("Open .obj file");
        hbox.add(&toggle_open_file);

        vbox.add(&hbox);

        // Actual display view
        let image = Image::new();
        vbox.add(&image);

        // All encompassing window
        let window = Window::new(WindowType::Toplevel);
        window.add(&vbox);
        window.show_all();

        // Create the raytrace framebuffer
        let fb = framebuffer::create_frame_buffer(800, 600);

        // Send the message Increment when the button is clicked.
        connect!(
            relm,
            toggle_render,
            connect_clicked(_),
            Msg::ToggleRendering
        );

        connect!(
            relm,
            toggle_open_file,
            connect_clicked(_),
            Msg::ToggleOpenFile
        );

        connect!(
            relm,
            toggle_save_to_file,
            connect_clicked(_),
            Msg::ToggleSaveToFile
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
    fn open_obj(&mut self, filepathbuf: std::path::PathBuf) {
        let path = filepathbuf.into_os_string().into_string();
        match path {
            Ok(filepath) => {
                // Make sure the renderer is started
                if !self.model.started_rendering.is_some() {
                    self.new_renderer()
                }

                // Load the file
                let objects = obj::load(filepath);

                // Add all the objects to the render scene
                let mut scene = scene::Scene::new();
                let off = geometry::Vec3f {
                    x: 0.,
                    y: 0.,
                    z: -500.,
                };

                if let Some(mut objects) = objects {
                    for mut obj in objects {
                        // `Box` moves storage to the heap
                        obj.offset(off);
                        scene.shapes.push(std::boxed::Box::new(obj));
                    }
                }
                println!["Opened file successfuly"];

                // Re-run the raytracer
                self.update_raytrace_image(&scene);
            }
            Err(e) => {
                println!["Filed opening .obj file. Error {:?}", e];
            }
        }
    }

    fn update_raytrace_image(&mut self, scene: &scene::Scene) {
        if self.model.started_rendering.is_some() {
            let raymarcher = self.model.started_rendering.as_mut().unwrap();
            raymarcher.render(&mut self.fb, scene);

            // Copy back the result in a gdk::pixbuff
            let image = &self.image;
            let pixbuf = Pixbuf::new_from_mut_slice(
                self.fb.to_vec(),
                gdk_pixbuf::Colorspace::Rgb,
                false,
                8,
                self.fb.width as i32,
                self.fb.height as i32,
                3 * self.fb.width as i32,
            );
            image.set_from_pixbuf(Some(&pixbuf));
            while gtk::events_pending() {
                gtk::main_iteration_do(true);
            }
        }
    }

    fn save_to_file(&mut self) {
        if self.model.started_rendering.is_some() {
            self.fb.normalize();
            self.fb.write_ppm("out.ppm").unwrap();
            self.state_label.set_text("Saved rendered file");
        } else {
            self.state_label
                .set_text("Cannot save, please start renderer");
        }
    }

    fn new_renderer(&mut self) {
        // Create the renderer
        // FIXME: the fb is only use for sizing purposes, should be cleaned
        let ray_marcher =
            renderer::create_renderer(1.5, self.fb.height as f64, self.fb.width as f64);
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
