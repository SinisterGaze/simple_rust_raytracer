#[macro_use]
extern crate impl_ops;
use float_cmp::{self, approx_eq};
use real_float;
use std::ops;

fn main() {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl_op_ex!(+ |lhs: &Vec3D, rhs: &Vec3D| -> Vec3D {Vec3D { x: (lhs.x+rhs.x), y: (lhs.y+rhs.y), z: (lhs.z+rhs.z) }});
impl_op_ex!(-|lhs: &Vec3D, rhs: &Vec3D| -> Vec3D {
    Vec3D {
        x: (lhs.x - rhs.x),
        y: (lhs.y - rhs.y),
        z: (lhs.z - rhs.z),
    }
});
impl_op_ex!(*|lhs: &Vec3D, rhs: &Vec3D| -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
});
impl_op_ex_commutative!(*|lhs: &Vec3D, rhs: &f64| -> Vec3D {
    Vec3D {
        x: (lhs.x * rhs),
        y: (lhs.y * rhs),
        z: (lhs.z * rhs),
    }
});
impl_op_ex!(/ |lhs: &Vec3D, rhs: &f64| -> Vec3D { Vec3D { x: (lhs.x / rhs), y: (lhs.y / rhs), z: (lhs.z / rhs) }});
impl_op_ex!(-|a: &Vec3D| -> Vec3D {
    Vec3D {
        x: (-a.x),
        y: (-a.y),
        z: (-a.z),
    }
});
impl_op_ex!(+= |lhs: &mut Vec3D, rhs: Vec3D| {lhs.x += rhs.x; lhs.y += rhs.y; lhs.z += rhs.z;});
impl_op_ex!(-= |lhs: &mut Vec3D, rhs: Vec3D| {lhs.x -= rhs.x; lhs.y -= rhs.y; lhs.z -= rhs.z;});
impl_op_ex!(*= |lhs: &mut Vec3D, rhs: &f64| {lhs.x *= rhs; lhs.y *= rhs; lhs.z *= rhs;});
impl_op_ex!(/= |lhs: &mut Vec3D, rhs: &f64| {lhs.x /= rhs; lhs.y /= rhs; lhs.z /= rhs;});

impl Vec3D {
    fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D { x, y, z}
    }

    fn norm(&self) -> f64 {
        f64::sqrt(self * self)
    }

    fn norm2(&self) -> f64 {
        self * self
    }

    fn normalize(&mut self) {
        (*self) /= self.norm();
    }

    fn unit_vector(&self) -> Self {
        let mut result = self.clone();
        result.normalize();
        result
    }
    fn cross(a: Vec3D, b: Vec3D) -> Vec3D {
        Vec3D { x: (a.y*b.z-a.z*b.y), y: -(a.x*b.z-a.z*b.x), z: (a.x*b.y-a.y*b.x) }
    }
}



impl std::fmt::Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl real_float::IsFinite for Vec3D {
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl std::default::Default for Vec3D {
    fn default() -> Self {
        Vec3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

pub trait Object3D {
    fn intersect(&self, ray: Ray) -> Option<Vec3D>;
}

pub struct Ray {
    origin: Vec3D,
    direction: Vec3D,
}

pub struct Sphere {
    center: Vec3D,
    radius: f64,
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
            let t = (-b - d.sqrt()) / (2.0*a);
            Some(ray.origin + t * ray.direction)
        }
    }
}

pub struct Plane {
    normal: Vec3D,
    distance: f64,
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
}

#[cfg(test)]
mod tests {
    use real_float::IsFinite;

    use crate::{Object3D, Plane, Vec3D, Ray, Sphere};

    #[test]
    fn test_addition() {
        let a = Vec3D::new(1.0, 0.0, 1.0);
        let b = Vec3D::new(2.0, 2.0, -1.0);
        assert_eq!(
            a + b,
            Vec3D::new(3.0, 2.0, 0.0)
        );
    }

    #[test]
    fn test_subtraction() {
        let a = Vec3D::new(1.0, 2.0, 3.0);
        let b = Vec3D::new(1.0, -1.0, 1.0);
        assert_eq!(
            a - b,
            Vec3D::new(0.0, 3.0, 2.0)
        );
    }

    #[test]
    fn test_scalar_mult() {
        let a = Vec3D::new(1.0, 2.0, 3.0);
        let b = a * 5.0;
        let c = 5.0 * a;
        assert_eq!(b, c);
        assert_eq!(
            b,
            Vec3D::new(5.0, 10.0, 15.0)
        );
    }

    #[test]
    fn test_scalar_div() {
        let a = Vec3D::new(2.0, 4.0, 6.0);
        let b = a / 2.0;
        assert_eq!(
            b,
            Vec3D::new(1.0, 2.0, 3.0)
        );
        assert!(!(a / 0.0).is_finite());
        let zero = Vec3D::default();
        assert!(!(zero / 0.0).is_finite());
    }

    #[test]
    fn test_dot_product() {
        let a = Vec3D::new(2.0, 4.0, 6.0);
        let b = Vec3D::new(1.0, -1.0, 1.0);
        assert_eq!(a * b, b * a);
        assert_eq!(a * b, 4.0);
    }

    #[test]
    fn test_cross_product() {
        let i = Vec3D::new(1.0, 0.0, 0.0);
        let j = Vec3D::new(0.0, 1.0, 0.0);
        let k = Vec3D::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3D::cross(i, j), k);
        assert_eq!(Vec3D::cross(j, k), i);
        assert_eq!(Vec3D::cross(k, i), j);

        assert_eq!(Vec3D::cross(j, i), -k);
        assert_eq!(Vec3D::cross(k, j), -i);
        assert_eq!(Vec3D::cross(i, k), -j);
    }
    #[test]
    fn test_norm() {
        let mut a = Vec3D::new(1.0, 2.0, 2.0);
        assert_eq!(a.norm(), 3.0); // sqrt(1*1 + 2*2 + 2*2) = sqrt(9) = 3
        let b = a.unit_vector();
        a.normalize();
        assert_eq!(b.norm(), 1.0); // norm of normalized vector is a unit vector
        assert_eq!(a.norm(), 1.0);
        let zero = Vec3D::default();
        assert_eq!(zero.norm(), 0.0); // norm of zero vector is zero
    }

    #[test]
    fn test_plane_intersect() {
        // Simple intersection of ray onto xy-plane
        let my_plane = Plane {
            normal: Vec3D::new(0.0, 0.0, 1.0),
            distance: 0.0,
        };

        let my_ray = Ray {
            origin: Vec3D::new(1.0, 2.0, -3.0),
            direction: Vec3D::new(0.0, 0.0, 1.0),
        };

        let intersection_point = my_plane.intersect(my_ray);
        assert_eq!(
            intersection_point.unwrap(),
            Vec3D::new(1.0, 2.0, 0.0)
        );

        // Test of ray parallel to plane
        // No (unique) intersection point
        let parallel_ray = Ray {
            origin: Vec3D::new(1.0, 2.0, -3.0),
            direction: Vec3D::new(-5.0, 7.0, 0.0),
        };
        let intersection_point = my_plane.intersect(parallel_ray);
        assert_eq!(intersection_point, None);

        // Test compared to example gathered from https://www.kristakingmath.com/blog/intersection-of-a-line-and-a-plane
        // Can't test for equality due to floating point precision, result is instead printed
        let my_plane = Plane {
            normal: Vec3D::new(2.0, -3.0, 1.0),
            distance: 3.0 / f64::sqrt(14.0),
        };

        let my_ray = Ray {
            origin: Vec3D::new(-1.0, 4.0, 1.0),
            direction: Vec3D::new(2.0, -5.0, 1.0),
        };
        #[allow(unused)]
        let intersection_point = my_plane.intersect(my_ray);
        //eprintln!("{}", intersection_point.unwrap()); // should be (0.6, 0, 1.8)
    }

    #[test]
    fn test_sphere_intersect() {
        let my_sphere = Sphere {
            center: Vec3D::default(),
            radius: 3.0,
        };
        let my_ray = Ray {
            origin: Vec3D::new(0.0, 0.0, 5.0),
            direction: Vec3D::new(0.0, 0.0, -1.0),
        };
        let intersection_point = my_sphere.intersect(my_ray).unwrap();
        //eprintln!("{}", intersection_point);
        assert_eq!(intersection_point, Vec3D::new(0.0, 0.0, 3.0));
    }
}
