use super::ray::Ray;
use crate::math::vector::Vec3D;
use crate::utils::color::Color;

pub trait Object3D {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectionData>;
    fn get_color() -> Color {
        todo!();
    }
}

#[derive(Debug, PartialEq)]
pub struct IntersectionData {
    pub t: f64,
    pub normal: Vec3D,
}