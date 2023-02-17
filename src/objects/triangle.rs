use crate::math::vector::Vec3D;
use crate::objects::{ray::Ray};
use crate::utils::fmod;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vert_a: Vec3D,
    pub vert_b: Vec3D,
    pub vert_c: Vec3D,
}

impl Triangle {
    pub fn normal(&self) -> Vec3D {
        let ab = self.vert_b - self.vert_a;
        let ac = self.vert_c - self.vert_a;

        Vec3D::cross(ab, ac).unit_vector()
    }

    pub fn get_intersection(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let normal = self.normal();
        let distance = self.vert_a * normal;

        if (normal * ray.direction).abs() <= f64::EPSILON {
            None
        } else {
            let t: f64 = (distance - ray.origin * normal) / (ray.direction * normal);
            if t_min < t && t < t_max {
                let p = ray.at(t);

                let test1: bool =
                    Vec3D::cross(self.vert_b - self.vert_a, p - self.vert_a) * normal >= 0.0;
                let test2: bool =
                    Vec3D::cross(self.vert_c - self.vert_b, p - self.vert_b) * normal >= 0.0;
                let test3: bool =
                    Vec3D::cross(self.vert_a - self.vert_c, p - self.vert_c) * normal >= 0.0;
                if test1 && test2 && test3 {
                    Some(t)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
    
    pub fn point_to_uv(&self, point: Vec3D) -> (f64, f64) {
        let normal = self.normal();
        let mut e1 = normal.cross(Vec3D::new(1.0, 0.0, 0.0));
        if e1.almost_zero() {
            e1 = normal.cross(Vec3D::new(0.0, 0.0, 1.0));
        }
        e1.normalize();
        let e2 = normal.cross(e1).unit_vector();
        let u = fmod(point * e1, 1.0);
        let v = fmod(point * e2, 1.0);
        (u, v)
    }
}
