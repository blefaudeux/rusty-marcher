use framebuffer::FrameBuffer;
use geometry::Vec3f;
use lights::Light;
use shapes::Shape;

pub struct Renderer {
    pub fov: f64,
    pub half_fov: f64,
    pub height: f64,
    pub width: f64,
    pub ratio: f64,
}

pub fn create_renderer(fov_: f64, frame: &FrameBuffer) -> Renderer {
    return Renderer {
        fov: fov_,
        half_fov: (fov_ / 2.).tan(),
        height: frame.height as f64,
        width: frame.width as f64,
        ratio: frame.width as f64 / frame.height as f64,
    };
}

impl Renderer {
    pub fn render(&self, frame: &mut FrameBuffer, shapes: Vec<&impl Shape>, _lights: Vec<&Light>) {
        let mut index = 0 as usize;

        for j in 0..frame.height {
            for i in 0..frame.width {
                let dir = self.backproject(i, j);
                frame.buffer[index] = cast_ray(dir, &shapes);
                index += 1;
            }
        }
    }

    fn backproject(&self, i: u32, j: u32) -> Vec3f {
        let mut dir = Vec3f {
            x: (2. * (i as f64 + 0.5) / self.width - 1.) * self.half_fov * self.ratio,
            y: -(2. * (j as f64 + 0.5) / self.height - 1.) * self.half_fov,
            z: -1.,
        };

        dir.normalize();

        return dir;
    }
}

fn cast_ray(dir: Vec3f, shapes: &Vec<&impl Shape>) -> Vec3f {
    let orig = Vec3f {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    for shape in shapes {
        let (valid, _distance) = shape.intersect(orig, dir);
        if valid {
            return Vec3f {
                x: 0.4,
                y: 0.4,
                z: 0.3,
            };
        }
    }

    return Vec3f {
        x: 0.2,
        y: 0.7,
        z: 0.8,
    }; // background color
}
