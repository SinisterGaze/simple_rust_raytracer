use crate::materials::PhongModel;
use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::Ray};

use crate::utils::fmod;

pub struct Plane {
    pub normal: Vec3D,
    pub distance: f64,
    pub phong_data: Option<PhongModel>,
}

impl Hittable for Plane {
    // Solve for ray: p0 + t * p intersecting with plane: n * v = d * |n| where
    // n = normal to plane
    // d = distance to plane from origin
    // v = vector on the plane
    // results in solution t = (d * |n| - r0 * n) / (r * n)
    // requires r * n =/= 0 for (unique) solution to exist (ray is not parallel with the plane)
    fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
        if (self.normal * ray.direction).abs() <= f64::EPSILON {
            None
        } else {
            let t: f64 = (self.distance * self.normal.norm() - ray.origin * self.normal)
                / (ray.direction * self.normal);

            if t_min < t && t < t_max {
                let normal = self.normal;
                let front_face = ray.direction * normal < 0.0;
                let (u, v) = self.point_to_uv(ray.at(t));
                Some(IntersectionData {
                    ray: ray,
                    t: t,
                    normal: if front_face { normal } else { -normal },
                    phong_data: self.phong_data.as_ref(),
                    u: u,
                    v: v,
                })
            } else {
                None
            }
        }
    }

    fn point_to_uv(&self, point: Vec3D) -> (f64, f64) {
        let mut e1 = self.normal.cross(Vec3D::new(1.0, 0.0, 0.0));
        if e1.almost_zero() {
            e1 = self.normal.cross(Vec3D::new(0.0, 0.0, 1.0));
        }
        e1.normalize();
        let e2 = self.normal.cross(e1).unit_vector();
        let u = fmod(point * e1, 1.0);
        let v = fmod(point * e2, 1.0);
        (u, v)
    }

    fn get_phong_data(&self) -> Option<&PhongModel> {
        self.phong_data.as_ref()
    }
}
