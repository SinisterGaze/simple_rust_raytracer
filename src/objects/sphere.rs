use float_cmp::approx_eq;

use crate::math::vector::Vec3D;
use crate::objects::{Object3D, Ray};
use crate::utils::color::Color;

pub struct Sphere {
    pub center: Vec3D,
    pub radius: f64,
    
}

impl Object3D for Sphere {
    // Solve for ray: p0 + t * p intersecting with sphere: |x-x0| = r
    // results in solving a quadratic formula at^2 + bt + c = 0 with
    // a = p^2
    // b = 2 * p * (p0 - c)
    // c = x0^2 + p0^2 - r^2 - 2 * p0 * c
    // requires D = b^2 - 4*ac >= 0 for solution(s) to exist
    // returns the solution closest to the origin of the ray
    // (assuming the ray's origin is outside of the sphere)
    // (assuming the sphere is in the +-direction of the ray)
    fn intersect(&self, ray: Ray) -> Option<Vec3D> {
        let a = ray.direction.norm2(); // a = r^2
        let b = 2.0 * (ray.direction * (ray.origin - self.center));
        let c = self.center.norm2() + ray.origin.norm2()
            - self.radius.powi(2)
            - 2.0 * (ray.origin * self.center);

        let d = b * b - 4.0 * a * c;
        if d < 0.0 || approx_eq!(f64, a, 0.0, ulps = 2) {
            None
        } else {
            let t = (-b - d.sqrt()) / (2.0 * a);
            Some(ray.origin + t * ray.direction)
        }
    }

    fn get_normat_at(&self, point: Vec3D) -> Vec3D {
        (point - self.center).unit_vector()
    }
}