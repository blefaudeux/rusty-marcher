use geometry;

use std::fs::File;
use std::io::Write;

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<geometry::Vec3f>,
}

pub fn create_frame_buffer(width_: u32, height_: u32) -> FrameBuffer {
    return FrameBuffer {
        width: width_,
        height: height_,
        buffer: vec![
            geometry::Vec3f {
                x: 0.,
                y: 0.,
                z: 0.
            };
            (width_ * height_) as usize
        ],
    };
}

impl FrameBuffer {
    pub fn write_ppm(&self, filename: &str) -> std::io::Result<usize> {
        // Open the file stream and dump
        let mut file = File::create(filename)?;
        // return file.write_all(buffer);

        // Standard PPM header
        file.write(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        // Write line by line, probably not needed thanks to buffering, but anyway..
        let mut writte_buffer = vec![0 as u8; (self.width * self.height * 3) as usize];
        let mut i_buffer = 0 as usize;
        let mut i_line = 0 as usize;

        for _i in 0..self.height {
            for _j in 0..self.width {
                writte_buffer[i_line..i_line + 3].clone_from_slice(&[
                    quantize(&self.buffer[i_buffer].x),
                    quantize(&self.buffer[i_buffer].y),
                    quantize(&self.buffer[i_buffer].z),
                ]);
                i_line += 3;
                i_buffer += 1;
            }
        }
        file.write(&writte_buffer)?;
        Ok(0)
    }

    // pub fn fill_gradient(&mut self) {
    //     let fh = self.height as f64;
    //     let fw = self.width as f64;
    //     let mut index = 0 as usize;

    //     for j in 0..self.height {
    //         for i in 0..self.width {
    //             self.buffer[index] = geometry::Vec3f {
    //                 x: (j as f64) / fh,
    //                 y: (i as f64) / fw,
    //                 z: 0 as f64,
    //             };
    //             index += 1;
    //         }
    //     }
    // }

    pub fn normalize(&mut self) {
        let mut index = 0 as usize;
        let mut max = geometry::Vec3f::zero();

        for _ in 0..self.height {
            for _ in 0..self.width {
                max.x = max.x.max(self.buffer[index].x);
                max.y = max.y.max(self.buffer[index].y);
                max.z = max.z.max(self.buffer[index].z);

                index += 1;
            }
        }

        let max_val = max.x.max(max.y).max(max.z);
        index = 0;
        if max_val > 0. {
            for _ in 0..self.height {
                for _ in 0..self.width {
                    self.buffer[index].scale(1. / max_val);
                    index += 1;
                }
            }
        }
    }
}

fn quantize(f: &f64) -> u8 {
    return (255. * f.max(0.).min(1.)) as u8;
}
