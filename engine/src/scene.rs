extern crate serde_json;
use geometry::Vec3f;
use lights;
use polygon;
use shapes::Reflectance;
use shapes::Shape;
use sphere;

pub struct Scene {
    pub lights: Vec<lights::Light>,
    pub shapes: Vec<Box<dyn Shape + Sync>>,
}

impl Scene {
    pub fn create_default() -> Scene {
        let mut reflectance = Reflectance::create_default();

        // Red sphere
        reflectance.diffuse_color = Vec3f {
            x: 0.8,
            y: 0.,
            z: 0.,
        };
        reflectance.specular_exponent = 100.;

        let sphere_red = sphere::create(
            Vec3f {
                x: -5.,
                y: 0.,
                z: -16.,
            },
            4.,
            reflectance,
        );

        // polygon
        reflectance.diffuse_color = Vec3f {
            x: 0.6,
            y: 0.,
            z: 0.7,
        };
        let triangle = polygon::create(
            vec![
                Vec3f {
                    x: 6.,
                    y: 3.,
                    z: -8.,
                },
                Vec3f {
                    x: 15.,
                    y: 0.,
                    z: -9.,
                },
                Vec3f {
                    x: 7.,
                    y: -4.,
                    z: -8.,
                },
            ],
            reflectance,
        );

        // Floor
        reflectance.diffusion = 1.0;
        reflectance.specular = 1.;
        reflectance.is_glass_like = true;
        reflectance.refractive_index = 1.5;
        reflectance.reflection = 0.5;
        reflectance.diffuse_color = Vec3f {
            x: 0.3,
            y: 0.9,
            z: 0.9,
        };

        let square = polygon::create(
            vec![
                Vec3f {
                    x: 15.,
                    y: -6.,
                    z: -3.,
                },
                Vec3f {
                    x: -15.,
                    y: -6.,
                    z: -3.,
                },
                Vec3f {
                    x: -20.,
                    y: -3.,
                    z: -50.,
                },
                Vec3f {
                    x: 20.,
                    y: -3.,
                    z: -50.,
                },
            ],
            reflectance,
        );

        // Blue sphere
        reflectance.specular = 1.0;
        reflectance.diffusion = 0.2;
        reflectance.diffuse_color = Vec3f {
            x: 0.,
            y: 0.,
            z: 0.2,
        };
        reflectance.is_glass_like = true;
        reflectance.refractive_index = 1.5;
        reflectance.reflection = 0.3;

        let sphere_blue = sphere::create(
            Vec3f {
                x: -0.5,
                y: -1.5,
                z: -5.,
            },
            2.,
            reflectance,
        );

        // Green sphere
        reflectance.diffusion = 1.;
        reflectance.reflection = 1.;
        reflectance.is_glass_like = false;
        reflectance.specular = 0.8;
        reflectance.diffuse_color = Vec3f {
            // Green sphere
            x: 0.,
            y: 1.,
            z: 0.,
        };

        let sphere_green = sphere::create(
            Vec3f {
                x: 6.,
                y: -0.5,
                z: -18.,
            },
            3.,
            reflectance,
        );

        // White sphere
        reflectance.diffuse_color = Vec3f {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        };
        let sphere_white = sphere::create(
            Vec3f {
                x: -10.,
                y: 6.,
                z: -14.,
            },
            4.,
            reflectance,
        );

        // Add a light to the scene
        let light_white = lights::create_light(
            Vec3f {
                x: -20.,
                y: 20.,
                z: 20.,
            },
            Vec3f::ones(), // white light
            1.,
        );

        let light_red = lights::create_light(
            Vec3f {
                x: 20.,
                y: 20.,
                z: 20.,
            },
            Vec3f {
                x: 1.,
                y: 0.5,
                z: 0.5,
            }, // reddish light
            0.8,
        );

        let scene = Scene {
            lights: vec![light_white, light_red],
            shapes: vec![
                Box::new(sphere_blue),
                Box::new(sphere_green),
                Box::new(sphere_red),
                Box::new(sphere_white),
                Box::new(triangle),
                Box::new(square),
            ],
        };
        return scene;
    }
}
