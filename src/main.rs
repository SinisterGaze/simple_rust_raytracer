#[macro_use] extern crate impl_ops;
pub use crate::math::geometry::{Object3D, Plane, Vec3D, Ray, Sphere};

pub mod math;

fn main() {

}

// (r, g, b) where 0 <= r, g, b <= 1
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub struct Light {
    pub position: Vec3D,
    pub color: Color,
}

pub struct Camera {
    pub origin: Vec3D,
    pub up: Vec3D,
    pub right: Vec3D,
}




