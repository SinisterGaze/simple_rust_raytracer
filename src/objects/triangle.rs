use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::Ray};

use image::Rgb;

pub struct Triangle {
    pub vert_a: Vec3D,
    pub vert_b: Vec3D,
    pub vert_c: Vec3D,
    pub color: Rgb<u8>,
}

impl Triangle {
    fn normal(&self) -> Vec3D {
        let ab = self.vert_b - self.vert_a;
        let ac = self.vert_c - self.vert_a;

        Vec3D::cross(ab, ac).unit_vector()
    }
}

impl Hittable for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
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
                    let front_face = ray.direction * normal < 0.0;
                    Some(IntersectionData {
                        t: t,
                        normal: if front_face { normal } else { -normal },
                        color: self.color,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
