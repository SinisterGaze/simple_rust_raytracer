use crate::math::vector::Vec3D;

use std::fs::File;
use std::io::prelude::*;

pub fn sphere_to_uv(point: Vec3D) -> Result<(f64, f64), &'static str> {
    if point.unit_vector() != point {
        Err("Vector must be a unit vector.")
    } else {
        use std::f64::consts::PI;
        let u = f64::atan2(point.x, point.z) / (2.0 * PI) + 0.5;
        let v = f64::asin(point.y) / PI + 0.5;
        Ok((u, v))
    }
}

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