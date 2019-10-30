use geometry;

use std::fs::File;
use std::io::Write;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec<geometry::Vec3f>>,
}

pub fn create_frame_buffer(width: usize, height: usize) -> FrameBuffer {
    // Pre-allocate the whole buffer
    let line_buffer: Vec<geometry::Vec3f> = vec![geometry::Vec3f::zero(); width];
    let buffer = vec![line_buffer; height];

    FrameBuffer {
        width,
        height,
        buffer, // height * lines
    }
}

impl FrameBuffer {
    pub fn write_ppm(&self, filename: &str) -> std::io::Result<usize> {
        // Open the file stream and dump
        let mut file = File::create(filename)?;
        // return file.write_all(buffer);

        // Standard PPM header
        file.write_all(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        // Write line by line, probably not needed thanks to buffering, but anyway..
        let write_buffer = self.to_vec();
        file.write_all(&write_buffer)?;
        Ok(0)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut write_buffer = vec![0 as u8; self.width * self.height * 3];
        let mut i_ = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                write_buffer[i_..i_ + 3].clone_from_slice(&[
                    quantize(self.buffer[i][j].x),
                    quantize(self.buffer[i][j].y),
                    quantize(self.buffer[i][j].z),
                ]);
                i_ += 3;
            }
        }

        return write_buffer;
    }

    pub fn normalize(&mut self) {
        let mut max = geometry::Vec3f::zero();

        for i in 0..self.height {
            for j in 0..self.width {
                max.x = max.x.max(self.buffer[i][j].x);
                max.y = max.y.max(self.buffer[i][j].y);
                max.z = max.z.max(self.buffer[i][j].z);
            }
        }

        let max_val = max.x.max(max.y).max(max.z);
        if max_val > 0. {
            for i in 0..self.height {
                for j in 0..self.width {
                    self.buffer[i][j].scale(1. / max_val);
                }
            }
        }
    }
}

fn quantize(f: f64) -> u8 {
    (255. * f.max(0.).min(1.)) as u8
}
