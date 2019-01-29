mod frame_to_disk;
mod geometry;

fn main() {
    // Allocate our dummy buffer
    let width = 1280 as u32;
    let height = 800 as u32;

    let mut frame_buffer = vec![
        geometry::Vec3f {
            x: 0.,
            y: 0.,
            z: 0.
        };
        (width * height) as usize
    ];

    // DUMMY
    // Write a pretty gradient for now
    let fh = height as f64;
    let fw = width as f64;
    let mut index = 0 as usize;
    for j in 0..height {
        for i in 0..width {
            frame_buffer[index] = geometry::Vec3f {
                x: (j as f64) / fh,
                y: (i as f64) / fw,
                z: 0 as f64,
            };
            index += 1;
        }
    }

    // Save to file
    frame_to_disk::write_ppm("out.ppm", &frame_buffer, &width, &height).unwrap();
}
