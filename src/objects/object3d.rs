use super::ray::Ray;
use crate::math::vector::Vec3D;
use crate::utils::color::Color;

pub trait Object3D {
    fn intersect(&self, ray: Ray) -> Option<Vec3D>;
    fn get_normat_at(&self, point: Vec3D) -> Vec3D;
    fn get_color() -> Color {
        todo!();
    }
}