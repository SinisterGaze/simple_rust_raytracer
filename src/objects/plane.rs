use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::Ray};

use image::Rgb;

pub struct Plane {
    pub normal: Vec3D,
    pub distance: f64,
    pub color: Rgb<u8>,
}

impl Hittable for Plane {
    // Solve for ray: p0 + t * p intersecting with plane: n * v = d * |n| where
    // n = normal to plane
    // d = distance to plane from origin
    // v = vector on the plane
    // results in solution t = (d * |n| - r0 * n) / (r * n)
    // requires r * n =/= 0 for (unique) solution to exist (ray is not parallel with the plane)
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
        if (self.normal * ray.direction).abs() <= f64::EPSILON {
            None
        } else {
            let t: f64 = (self.distance * self.normal.norm() - ray.origin * self.normal)
                / (ray.direction * self.normal);

            if t_min < t && t < t_max {
                let normal = self.normal;
                let front_face = ray.direction * normal < 0.0;
                Some(IntersectionData {
                    t: t,
                    normal: if front_face { normal } else { -normal },
                    color: self.color,
                })
            } else {
                None
            }
        }
    }
}
