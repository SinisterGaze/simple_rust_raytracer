use crate::materials::PhongModel;
use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::Ray};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3D,
    pub radius: f64,
    pub phong_data: Option<PhongModel>,
}

impl Hittable for Sphere {
    // Solve for ray: p0 + t * p intersecting with sphere: |x-x0| = r
    // results in solving a quadratic formula at^2 + bt + c = 0 with
    // a = p^2
    // b = 2 * p * (p0 - c)
    // c = x0^2 + p0^2 - r^2 - 2 * p0 * c
    // requires D = b^2 - 4*ac >= 0 for solution(s) to exist
    // returns the solution closest to the origin of the ray
    // (assuming the ray's origin is outside of the sphere)
    // (assuming the sphere is in the +-direction of the ray)
    fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
        let a = ray.direction.norm2(); // a = r^2
        let b_half = ray.direction * (ray.origin - self.center);
        let c = self.center.norm2() + ray.origin.norm2()
            - self.radius.powi(2)
            - 2.0 * (ray.origin * self.center);

        let discriminant = b_half * b_half - a * c;
        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let root1 = (-b_half - sqrt_discriminant) / a;
            let root2 = (-b_half + sqrt_discriminant) / a;
            let is_within_bounds = |t| -> bool { t_min < t && t < t_max };
            let mut root = root1; // root1 <= root2
            if !is_within_bounds(root1) {
                root = root2;
                if !is_within_bounds(root2) {
                    return None;
                }
            }

            let p: Vec3D = ray.at(root);
            let normal: Vec3D = (p - self.center) / self.radius;
            let front_face: bool = ray.direction * normal < 0.0;
            let (u, v) = self.point_to_uv(normal);
            return Some(IntersectionData {
                ray: ray,
                t: root,
                normal: if front_face { normal } else { -normal },
                phong_data: self.phong_data.as_ref(),
                u: u,
                v: v,
            });
        } else {
            None
        }
    }

    fn get_phong_data(&self) -> Option<&PhongModel> {
        self.phong_data.as_ref()
    }

    fn point_to_uv(&self, point: Vec3D) -> (f64, f64) {
        use std::f64::consts::PI;
        let u = f64::atan2(point.x, point.z) / (2.0 * PI) + 0.5;
        let v = f64::asin(point.y) / PI + 0.5;
        (u, 1.0 - v)
    }
}
