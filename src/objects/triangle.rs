use crate::math::vector::Vec3D;
use crate::objects::{Object3D, Ray};
use crate::utils::color::Color;

pub struct Triangle {
    pub vert_a: Vec3D,
    pub vert_b: Vec3D,
    pub vert_c: Vec3D,
}

impl Object3D for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Vec3D> {
        let normal = self.get_normat_at(ray.origin);
        let distance = self.vert_a * normal;

        if float_cmp::approx_eq!(f64, normal * ray.direction, 0.0, ulps = 2) {
            None
        } else {
            let t: f64 = (distance - ray.origin * normal) / (ray.direction * normal);
            let p = ray.origin + t * ray.direction;

            let test1: bool = Vec3D::cross(self.vert_b - self.vert_a, p - self.vert_a) * normal >= 0.0;
            let test2: bool = Vec3D::cross(self.vert_c - self.vert_b, p - self.vert_b) * normal >= 0.0;
            let test3: bool = Vec3D::cross(self.vert_a - self.vert_c, p - self.vert_c) * normal >= 0.0;

            if test1 && test2 && test3 {
                Some(p)
            } else {
                None
            }
        }
    }

    #[allow(unused)]
    fn get_normat_at(&self, point: Vec3D) -> Vec3D {
        let ab = self.vert_b - self.vert_a;
        let ac = self.vert_c - self.vert_a;

        Vec3D::cross(ab, ac).unit_vector()
    }
}