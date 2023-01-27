#[macro_use]
extern crate relm;

#[macro_use]
extern crate relm_derive;

extern crate gdk;
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
use gtk::prelude::*;
use gtk::*;
use relm::{Relm, Update, Widget};

#[allow(dead_code)]
struct Model {
    relm: Relm<Win>,
    started_rendering: Option<renderer::Renderer>,
}

#[derive(Msg, Clone, Copy)]
enum Msg {
    Quit,
    ToggleDefaultScene,
    ToggleOpenFile,
    ToggleMoveBack,
    ToggleMoveCloser,
    ToggleMoveLeft,
    ToggleMoveRight,
    ToggleMoveUp,
    ToggleMoveDown,
    ToggleSaveToFile,
    Sink,
}

// Create the structure that holds the widgets used in the view.
struct Win {
    state_label: Label,
    image: Image,
    model: Model,
    window: Window,
    fb: framebuffer::FrameBuffer,
    scene: scene::Scene,
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
        let mut update_camera = |offset: geometry::Vec3f| {
            self.scene.offset_camera(offset);
            self.update_raytrace_image();
        };

        match event {
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
                let path = open_file_dialog.filename();
                unsafe {
                    open_file_dialog.destroy();
                }

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
            Msg::Sink => {}
            Msg::ToggleDefaultScene => {
                self.scene = scene::Scene::create_default();
                self.new_renderer();
                self.update_raytrace_image();
            }
            Msg::ToggleMoveBack => {
                let offset = geometry::Vec3f {
                    x: 0.0,
                    y: 0.0,
                    z: 5.0,
                };
                update_camera(offset);
            }
            Msg::ToggleMoveCloser => {
                let offset = geometry::Vec3f {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                };
                update_camera(offset);
            }
            Msg::ToggleMoveLeft => {
                let offset = geometry::Vec3f {
                    x: -5.0,
                    y: 0.0,
                    z: 0.0,
                };
                update_camera(offset);
            }
            Msg::ToggleMoveRight => {
                let offset = geometry::Vec3f {
                    x: 5.0,
                    y: 0.0,
                    z: 0.0,
                };
                update_camera(offset);
            }
            Msg::ToggleMoveUp => {
                let offset = geometry::Vec3f {
                    x: 0.0,
                    y: 5.0,
                    z: 0.0,
                };
                update_camera(offset);
            }
            Msg::ToggleMoveDown => {
                let offset = geometry::Vec3f {
                    x: 0.0,
                    y: -5.0,
                    z: 0.0,
                };
                update_camera(offset);
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

        // -- quick lambda to automate button declaration and binding
        let add_button = |container: &gtk::Box, label: &str, event: Msg| {
            let toggle = Button::with_label(label);
            container.add(&toggle);
            connect!(relm, toggle, connect_clicked(_), event);
        };
        // -- add all the buttons we're interested in
        add_button(&hbox, "Default scene", Msg::ToggleDefaultScene);
        add_button(&hbox, "Save to file", Msg::ToggleSaveToFile);
        add_button(&hbox, "Open file", Msg::ToggleOpenFile);

        add_button(&hbox, "Left", Msg::ToggleMoveLeft);
        add_button(&hbox, "Right", Msg::ToggleMoveRight);
        add_button(&hbox, "Up", Msg::ToggleMoveUp);
        add_button(&hbox, "Down", Msg::ToggleMoveDown);
        vbox.add(&hbox);

        // Actual display view
        let image = Image::new();
        vbox.add(&image);

        // All encompassing window
        let window = Window::new(WindowType::Toplevel);
        window.add(&vbox);
        window.show_all();

        // Grab the mouse events
        window.add_events(gdk::EventMask::SCROLL_MASK);

        connect!(
            relm,
            window,
            connect_scroll_event(_, evt),
            return (
                if evt.direction() == gdk::ScrollDirection::Up {
                    Msg::ToggleMoveBack
                } else if evt.direction() == gdk::ScrollDirection::Down {
                    Msg::ToggleMoveCloser
                } else {
                    Msg::Sink
                },
                Inhibit(false)
            )
        );

        // Create the raytrace framebuffer
        let fb = framebuffer::create_frame_buffer(1600, 1280);

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
            scene: scene::Scene::create_default(),
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

                if let Some(objects) = objects {
                    // Default offset, move the scene to the back
                    let offset = geometry::Vec3f {
                        x: 0.,
                        y: 0.,
                        z: -500.,
                    };

                    for mut obj in objects {
                        obj.offset(offset);
                        // `Box` moves storage to the heap
                        scene.shapes.push(std::boxed::Box::new(obj));
                    }
                }

                // Add an arbitrary set of lights
                // FIXME: Need something a tiny bit better
                scene.lights.push(lights::create_light(
                    geometry::Vec3f {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    },
                    geometry::Vec3f::ones(), // white light
                    1.,
                ));

                scene.lights.push(lights::create_light(
                    geometry::Vec3f {
                        x: 20.,
                        y: 20.,
                        z: 20.,
                    },
                    geometry::Vec3f {
                        x: 1.,
                        y: 0.5,
                        z: 0.5,
                    }, // reddish light
                    0.8,
                ));

                println!["Opened file successfuly"];
                self.scene = scene;

                // Re-run the raytracer
                self.update_raytrace_image();
            }
            Err(e) => {
                println!["Filed opening .obj file. Error {:?}", e];
            }
        }
    }

    fn update_raytrace_image(&mut self) {
        if self.model.started_rendering.is_some() {
            let raymarcher = self.model.started_rendering.as_mut().unwrap();
            self.state_label
                .set_text(&raymarcher.render(&mut self.fb, &self.scene).to_string());

            // Copy back the result in a gdk::pixbuff
            let image = &self.image;
            let pixbuf = Pixbuf::from_mut_slice(
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

    #[allow(dead_code)]
    fn clear_renderer(&mut self) {
        self.model.started_rendering = None;
        self.state_label.set_text("Cleared renderer");
    }
}

fn main() {
    Win::run(()).unwrap();
}
