pub mod ray;
pub mod plane;
pub mod sphere;
pub mod triangle;

use crate::objects::ray::Ray;
use crate::math::vector::Vec3D;
use crate::utils::color::Color;

pub trait Object3D {
    fn intersect(&self, ray: Ray) -> Option<Vec3D>;
    fn get_normat_at(&self, point: Vec3D) -> Vec3D;
    fn get_color() -> Color {
        todo!();
    }
}











