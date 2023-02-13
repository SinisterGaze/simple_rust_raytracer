use crate::math::vector::Vec3D;

pub mod camera;
pub mod light;

fn sphere_to_uv(point: Vec3D) -> Result<(f64, f64), &'static str> {
    if point.unit_vector() != point {
        Err("Vector must be a unit vector.")
    } else {
        use std::f64::consts::PI;
        let u = f64::atan2(point.x, point.z)/(2.0 * PI) + 0.5;
        let v = f64::asin(point.y)/PI + 0.5;
        Ok((u, v))
    }
}



