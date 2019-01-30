use geometry;

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
