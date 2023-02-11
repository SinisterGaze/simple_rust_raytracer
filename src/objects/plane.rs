use crate::math::vector::Vec3D;
use crate::objects::{Object3D, Ray};
use crate::utils::color::Color;

pub struct Plane {
    pub normal: Vec3D,
    pub distance: f64,
}

impl Object3D for Plane {
    // Solve for ray: p0 + t * p intersecting with plane: n * v = d * |n| where
    // n = normal to plane
    // d = distance to plane from origin
    // v = vector on the plane
    // results in solution t = (d * |n| - r0 * n) / (r * n)
    // requires r * n =/= 0 for (unique) solution to exist (ray is not parallel with the plane)
    fn intersect(&self, ray: Ray) -> Option<Vec3D> {
        if float_cmp::approx_eq!(f64, self.normal * ray.direction, 0.0, ulps = 2) {
            None
        } else {
            let t: f64 = (self.distance * self.normal.norm() - ray.origin * self.normal)
                / (ray.direction * self.normal);
            Some(ray.origin + t * ray.direction)
        }
    }

    #[allow(unused)]
    fn get_normat_at(&self, point: Vec3D) -> Vec3D {
        return self.normal;
    }
}