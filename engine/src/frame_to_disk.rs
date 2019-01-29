use std::fs::File;
use std::io::Write;

use geometry::Vec3f;

fn quantize(f: &f64) -> u8 {
    return (255. * f.max(0.).min(1.)) as u8;
}

pub fn write_ppm(
    filename: &str,
    buffer: &Vec<Vec3f>,
    width: &u32,
    height: &u32,
) -> std::io::Result<usize> {
    // Failsafe, check dimensions
    assert_eq!(buffer.len() as u32, width * height);

    // Open the file stream and dump
    let mut file = File::create(filename)?;
    // return file.write_all(buffer);

    // Standard PPM header
    file.write(format!("P6\n{} {}\n255\n", width, height).as_bytes())?;

    // Write line by line, probably not needed thanks to buffering, but anyway..
    let mut line = vec![0 as u8; (*width * 3) as usize];
    let mut i_buffer = 0 as usize;
    let mut i_line = 0 as usize;

    for _i in 0..*height {
        for _j in 0..*width {
            line[i_line..i_line + 3].clone_from_slice(&[
                quantize(&buffer[i_buffer].x),
                quantize(&buffer[i_buffer].y),
                quantize(&buffer[i_buffer].z),
            ]);
            i_line += 3;
            i_buffer += 1;
        }
        i_line = 0;
        file.write(&line)?;
    }
    Ok(0)
}
