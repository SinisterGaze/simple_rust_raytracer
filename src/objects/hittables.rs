use crate::materials::PhongModel;
use crate::math::vector::Vec3D;
use crate::objects::ray::Ray;

pub trait Hittable {
    fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData>;

    fn get_phong_data(&self) -> &PhongModel;

    fn point_to_uv(&self, point: Vec3D) -> (f64, f64);
}

#[derive(Debug)]
pub struct IntersectionData<'a> {
    pub ray: Ray,
    pub t: f64,
    pub normal: Vec3D,
    pub phong_data: &'a PhongModel,
    pub u: f64,
    pub v: f64,
}
