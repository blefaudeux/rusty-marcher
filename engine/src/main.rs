extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate log;

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
use gio::prelude::*;
use gtk::prelude::*;
use std::sync::{Arc, RwLock};
use std::thread;
use {std::sync::mpsc::channel, std::sync::mpsc::Receiver, std::sync::mpsc::Sender};

#[derive(Clone, Copy)]
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
struct RenderBackbone {
    // This is NOT thread safe by any means,
    // consider wrapping it in a RwLock if needed
    state_label: gtk::Label,
    image: gtk::Image,
    fb: framebuffer::FrameBuffer,
    renderer: Option<renderer::Renderer>,
    scene: scene::Scene,
}

impl RenderBackbone {
    pub fn new() -> Self {
        // Create the raytrace framebuffer
        RenderBackbone {
            state_label: gtk::Label::new(Some("Waiting to create the renderer")),
            image: gtk::Image::new(),
            fb: framebuffer::create_frame_buffer(1280, 800),
            renderer: None,
            scene: scene::Scene::create_default(),
        }
    }

    pub fn open_obj(&mut self, filepathbuf: std::path::PathBuf) {
        let path = filepathbuf.into_os_string().into_string();
        match path {
            Ok(filepath) => {
                // Make sure the renderer is started
                if !self.renderer.is_some() {
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

    pub fn update_raytrace_image(&mut self) {
        if self.renderer.is_some() {
            let raymarcher = self.renderer.as_mut().unwrap();
            self.state_label
                .set_text(&raymarcher.render(&mut self.fb, &self.scene).to_string());

            // Copy back the result in a gdk::pixbuff
            let pixbuf = Pixbuf::new_from_mut_slice(
                self.fb.to_vec(),
                gdk_pixbuf::Colorspace::Rgb,
                false,
                8,
                self.fb.width as i32,
                self.fb.height as i32,
                3 * self.fb.width as i32,
            );
            self.image.set_from_pixbuf(Some(&pixbuf));
            while gtk::events_pending() {
                gtk::main_iteration_do(true);
            }
        }
    }

    pub fn save_to_file(&mut self) {
        if self.renderer.is_some() {
            self.fb.normalize();
            self.fb.write_ppm("out.ppm").unwrap();

            self.state_label.set_text("Saved rendered file");
        } else {
            self.state_label
                .set_text("Cannot save, please start renderer");
        }
    }

    pub fn new_renderer(&mut self) {
        // Create the renderer
        // FIXME: the fb is only use for sizing purposes, should be cleaned

        self.renderer = Some(renderer::create_renderer(
            1.5,
            self.fb.height as f64,
            self.fb.width as f64,
        ));
        self.state_label.set_text("Created new ray tracing engine");
    }

    #[allow(dead_code)]
    pub fn clear_renderer(&mut self) {
        self.renderer = None;
        self.state_label.set_text("Cleared renderer");
    }
}

fn build_ui(
    backbone: &Arc<RwLock<RenderBackbone>>,
    app: &gtk::Application,
    sender: &Arc<Sender<Msg>>,
) -> gtk::ApplicationWindow {
    // All encompassing window
    let window = gtk::ApplicationWindow::new(app);
    window.add_events(gdk::EventMask::SCROLL_MASK | gdk::EventMask::BUTTON_PRESS_MASK);

    // Create the view using the normal GTK+ method calls.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // vbox.add(&self.state_label.read().unwrap());

    // - toolbar
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    // -- quick lambda to automate button declaration and binding
    let add_button = |container: &gtk::Box, label: &str, msg: Msg, pipe: &Arc<Sender<Msg>>| {
        let toggle = gtk::Button::new_with_label(label);
        // Clone the shared pointer, needs to be owned per lambda instance
        let pipe = pipe.clone();
        toggle.connect_button_press_event(move |_, _| {
            pipe.send(msg).unwrap();
            gtk::Inhibit(false)
        });
        container.add(&toggle);
    };

    // -- add all the buttons we're interested in
    {
        add_button(
            &hbox,
            "Default scene",
            Msg::ToggleDefaultScene,
            &sender.clone(),
        );
        add_button(
            &hbox,
            "Save to file",
            Msg::ToggleSaveToFile,
            &sender.clone(),
        );
        add_button(&hbox, "Open file", Msg::ToggleOpenFile, &sender.clone());

        add_button(&hbox, "Left", Msg::ToggleMoveLeft, &sender.clone());
        add_button(&hbox, "Right", Msg::ToggleMoveRight, &sender.clone());
        add_button(&hbox, "Up", Msg::ToggleMoveUp, &sender.clone());
        add_button(&hbox, "Down", Msg::ToggleMoveDown, &sender.clone());
        vbox.add(&hbox);
        vbox.add(&backbone.read().unwrap().image);
        window.add(&vbox);
    }

    // Handle keyboard events
    window.connect_key_press_event(move |win, key| {
        if let Some(key) = gdk::keyval_name(key.get_keyval()) {
            match key.as_str() {
                "q" => {
                    if let Some(app) = win.get_application() {
                        log::info!("quitting!");
                        app.quit()
                    }
                }
                "t" => { /* TODO keep track if we're above or not */ }
                _ => {}
            };
        }
        gtk::Inhibit(false)
    });

    // Handle mouse events
    {
        let pipe = sender.clone();
        window.connect_scroll_event(move |_, ev| {
            if ev.get_direction() == gdk::ScrollDirection::Up {
                pipe.send(Msg::ToggleMoveBack).unwrap();
            } else if ev.get_direction() == gdk::ScrollDirection::Down {
                pipe.send(Msg::ToggleMoveCloser).unwrap();
            }
            Inhibit(false)
        });
    }

    {
        let pipe = sender.clone();
        window.connect_delete_event(move |_, _| {
            pipe.send(Msg::Quit).unwrap();
            Inhibit(false)
        });
    }

    window
}

fn dispatch(backbone: &Arc<RwLock<RenderBackbone>>, receiver: Receiver<Msg>) {
    let update_camera = |offset: geometry::Vec3f| {
        let mut w_backbone = backbone.write().unwrap();
        w_backbone.scene.offset_camera(offset);
        w_backbone.update_raytrace_image();
    };

    let handle_event = |event: Msg| {
        match event {
            Msg::ToggleSaveToFile => {
                backbone.write().unwrap().save_to_file();
            }
            Msg::Quit => gtk::main_quit(),
            Msg::ToggleOpenFile => {
                return;
                // // Open file dialog
                // let open_file_dialog = FileChooserDialog::with_buttons(
                //     Some("Open .obj file"),
                //     Some(&self.core),
                //     FileChooserAction::Open,
                //     &[
                //         ("_Cancel", ResponseType::Cancel),
                //         ("_Open", ResponseType::Accept),
                //     ],
                // );

                // // Filter: we just want .obj files
                // let filter = FileFilter::new();
                // filter.add_pattern("*.obj");
                // open_file_dialog.set_filter(&filter);

                // let button_pressed = open_file_dialog.run();
                // let path = open_file_dialog.get_filename();
                // open_file_dialog.destroy();

                // match path {
                //     Some(filepath) => {
                //         println!("Got {:?} from FileChooserDialog", filepath);
                //         if button_pressed == ResponseType::Accept.into() {
                //             self.open_obj(filepath);
                //         }
                //     }
                //     None => {}
                // }
            }
            Msg::Sink => {}
            Msg::ToggleDefaultScene => {
                let mut w_backbone = backbone.write().unwrap();
                w_backbone.scene = scene::Scene::create_default();
                w_backbone.new_renderer();
                w_backbone.update_raytrace_image();
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
    };

    // thread::spawn(move || {
    //     while true {
    let event = receiver.recv().unwrap();
    handle_event(event);
    //     }
    // );
}

fn main() {
    // Start up the GTK3 subsystem.
    gtk::init().expect("Unable to start GTK3. Error");

    // Create the actual app
    let application = gtk::Application::new(Some("rusty.marcher"), Default::default())
        .expect("failed to initialize GTK application");

    // Handle all callbacks
    let (sender, receiver) = channel::<Msg>();
    let sender = Arc::new(sender);

    let backbone = Arc::new(RwLock::<RenderBackbone>::new(RenderBackbone::new()));

    // Hook up to references of the different objects, these will survive the
    // scope through the move into the returned struct
    {
        let backbone_callback = backbone.clone();
        dispatch(&backbone_callback, receiver);
    }

    // Activate and run
    {
        let backbone_ui = backbone.clone();
        let sender_ui = sender.clone();
        application.connect_activate(move |app| {
            build_ui(&backbone_ui, &app, &sender_ui).show_all();
        });
    }

    application.run(&[]);
}
