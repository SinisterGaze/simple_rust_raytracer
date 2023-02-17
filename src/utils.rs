use std::fs::File;
use std::io::prelude::*;

pub fn save_ppm(
    filename: &str,
    width: u32,
    height: u32,
    pixels: &[u8],
) -> Result<(), std::io::Error> {
    assert_eq!(&filename[filename.len() - 4..], ".ppm");
    let mut output = File::create(filename)?;
    output.write_all(format!("P6\n{width} {height}\n255\n").as_bytes())?;
    output.write_all(pixels)?;
    Ok(())
}

pub fn fmod(num: f64, den: f64) -> f64 {
    let result = num % den;
    if result > 0.0 { result } else { den - result }
}