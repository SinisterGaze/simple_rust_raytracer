use super::ray::Ray;
use crate::math::vector::Vec3D;

use image::Rgb;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectionData>;
}

#[derive(Debug, PartialEq)]
pub struct IntersectionData {
    pub t: f64,
    pub normal: Vec3D,
    pub color: Rgb<u8>,
}
