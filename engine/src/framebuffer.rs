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

pub fn fill_gradient(frame: &mut FrameBuffer) {
    let fh = frame.height as f64;
    let fw = frame.width as f64;
    let mut index = 0 as usize;

    for j in 0..frame.height {
        for i in 0..frame.width {
            frame.buffer[index] = geometry::Vec3f {
                x: (j as f64) / fh,
                y: (i as f64) / fw,
                z: 0 as f64,
            };
            index += 1;
        }
    }
}

fn quantize(f: &f64) -> u8 {
    return (255. * f.max(0.).min(1.)) as u8;
}

impl FrameBuffer {
    pub fn write_ppm(&self, filename: &str) -> std::io::Result<usize> {
        // Open the file stream and dump
        let mut file = File::create(filename)?;
        // return file.write_all(buffer);

        // Standard PPM header
        file.write(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        // Write line by line, probably not needed thanks to buffering, but anyway..
        let mut line = vec![0 as u8; (self.width * 3) as usize];
        let mut i_buffer = 0 as usize;
        let mut i_line = 0 as usize;

        for _i in 0..self.height {
            for _j in 0..self.width {
                line[i_line..i_line + 3].clone_from_slice(&[
                    quantize(&self.buffer[i_buffer].x),
                    quantize(&self.buffer[i_buffer].y),
                    quantize(&self.buffer[i_buffer].z),
                ]);
                i_line += 3;
                i_buffer += 1;
            }
            i_line = 0;
            file.write(&line)?;
        }
        Ok(0)
    }
}
