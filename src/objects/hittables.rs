use crate::materials::Material;
use crate::math::vector::Vec3D;
use crate::objects::ray::Ray;

pub trait Hittable {
    fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData>;

    fn get_material(&self) -> Material;
}

#[derive(Debug, PartialEq)]
pub struct IntersectionData {
    pub ray: Ray,
    pub t: f64,
    pub normal: Vec3D,
    pub material: Material,
}
