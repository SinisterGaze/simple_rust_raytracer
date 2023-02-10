#[macro_use]
extern crate impl_ops;
use float_cmp::{self, approx_eq};
use real_float;
use std::ops;

fn main() {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl_op_ex!(+ |lhs: &Point3D, rhs: &Point3D| -> Point3D {Point3D { x: (lhs.x+rhs.x), y: (lhs.y+rhs.y), z: (lhs.z+rhs.z) }});
impl_op_ex!(-|lhs: &Point3D, rhs: &Point3D| -> Point3D {
    Point3D {
        x: (lhs.x - rhs.x),
        y: (lhs.y - rhs.y),
        z: (lhs.z - rhs.z),
    }
});
impl_op_ex!(*|lhs: &Point3D, rhs: &Point3D| -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
});
impl_op_ex_commutative!(*|lhs: &Point3D, rhs: &f64| -> Point3D {
    Point3D {
        x: (lhs.x * rhs),
        y: (lhs.y * rhs),
        z: (lhs.z * rhs),
    }
});
impl_op_ex!(/ |lhs: &Point3D, rhs: &f64| -> Point3D { Point3D { x: (lhs.x / rhs), y: (lhs.y / rhs), z: (lhs.z / rhs) }});
impl_op_ex!(-|a: &Point3D| -> Point3D {
    Point3D {
        x: (-a.x),
        y: (-a.y),
        z: (-a.z),
    }
});
impl_op_ex!(+= |lhs: &mut Point3D, rhs: Point3D| {lhs.x += rhs.x; lhs.y += rhs.y; lhs.z += rhs.z;});
impl_op_ex!(-= |lhs: &mut Point3D, rhs: Point3D| {lhs.x -= rhs.x; lhs.y -= rhs.y; lhs.z -= rhs.z;});
impl_op_ex!(*= |lhs: &mut Point3D, rhs: &f64| {lhs.x *= rhs; lhs.y *= rhs; lhs.z *= rhs;});
impl_op_ex!(/= |lhs: &mut Point3D, rhs: &f64| {lhs.x /= rhs; lhs.y /= rhs; lhs.z /= rhs;});

impl Point3D {
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
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl real_float::IsFinite for Point3D {
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

pub trait Object3D {
    fn intersect(&self, ray: Ray) -> Option<Point3D>;
}

pub struct Ray {
    point: Point3D,
    direction: Point3D,
}

pub struct Sphere {
    center: Point3D,
    radius: f64,
}

// Solve for ray: p0 + t * p intersecting with sphere: |x-x0| = r
// results in solving a quadratic formula at^2 + bt + c = 0 with
// a = p^2
// b = 2 * p * (p0 - c)
// c = x0^2 + p0^2 - r^2 - 2 * p0 * c
impl Object3D for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Point3D> {
        let a = ray.direction.norm2(); // a = r^2
        let b = 2.0 * (ray.direction * (ray.point - self.center));
        let c = self.center.norm2() + ray.point.norm2()
            - self.radius.powi(2)
            - 2.0 * (ray.point * self.center);

        let D = b * b - 4.0 * a * c;
        if D < 0.0 || approx_eq!(f64, a, 0.0, ulps = 2) {
            None
        } else {
            let t = (-b - D.sqrt())/(2.0*a);
            Some(ray.point + t * ray.direction)
        }
    }
}

pub struct Plane {
    normal: Point3D,
    distance: f64,
}

impl Object3D for Plane {
    fn intersect(&self, ray: Ray) -> Option<Point3D> {
        if float_cmp::approx_eq!(f64, self.normal * ray.direction, 0.0, ulps = 2) {
            None
        } else {
            let t: f64 = (self.distance * self.normal.norm() - ray.point * self.normal)
                / (ray.direction * self.normal);
            Some(ray.point + t * ray.direction)
        }
    }
}

#[cfg(test)]
mod tests {
    use real_float::IsFinite;

    use crate::{Object3D, Plane, Point3D, Ray, Sphere};

    #[test]
    fn test_addition() {
        let a = Point3D {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        };
        let b = Point3D {
            x: 2.0,
            y: 2.0,
            z: -1.0,
        };
        assert_eq!(
            a + b,
            Point3D {
                x: 3.0,
                y: 2.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn test_subtraction() {
        let a = Point3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Point3D {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        };
        assert_eq!(
            a - b,
            Point3D {
                x: 0.0,
                y: 3.0,
                z: 2.0,
            }
        );
    }

    #[test]
    fn test_scalar_mult() {
        let a = Point3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = a * 5.0;
        let c = 5.0 * a;
        assert_eq!(b, c);
        assert_eq!(
            b,
            Point3D {
                x: 5.0,
                y: 10.0,
                z: 15.0
            }
        );
    }

    #[test]
    fn test_scalar_div() {
        let a = Point3D {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        let b = a / 2.0;
        assert_eq!(
            b,
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
        assert!(!(a / 0.0).is_finite());
        let zero = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert!(!(zero / 0.0).is_finite());
    }

    #[test]
    fn test_dot_product() {
        let a = Point3D {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        let b = Point3D {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        };
        assert_eq!(a * b, b * a);
        assert_eq!(a * b, 4.0);
    }

    #[test]
    fn test_norm() {
        let a = Point3D {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(a.norm(), 3.0); // sqrt(1*1 + 2*2 + 2*2) = sqrt(9) = 3
        let b = a.unit_vector();
        assert_eq!(b.norm(), 1.0); // norm of normalized vector is a unit vector
        let zero = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(zero.norm(), 0.0); // norm of zero vector is zero
    }

    #[test]
    fn test_plane_intersect() {
        // Simple intersection of ray onto xy-plane
        let my_plane = Plane {
            normal: Point3D {
                x: (0.0),
                y: (0.0),
                z: (1.0),
            },
            distance: 0.0,
        };

        let my_ray = Ray {
            point: Point3D {
                x: (1.0),
                y: (2.0),
                z: (-3.0),
            },
            direction: Point3D {
                x: (0.0),
                y: (0.0),
                z: (1.0),
            },
        };

        let intersection_point = my_plane.intersect(my_ray);
        assert_eq!(
            intersection_point.unwrap(),
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 0.0
            }
        );

        // Test of ray parallel to plane
        // No (unique) intersection point
        let parallel_ray = Ray {
            point: Point3D {
                x: (1.0),
                y: (2.0),
                z: (-3.0),
            },
            direction: Point3D {
                x: (-5.0),
                y: (7.0),
                z: (0.0),
            },
        };
        let intersection_point = my_plane.intersect(parallel_ray);
        assert_eq!(intersection_point, None);

        // Test compared to example gathered from https://www.kristakingmath.com/blog/intersection-of-a-line-and-a-plane
        // Can't test for equality due to floating point precision, result is instead printed
        let my_plane = Plane {
            normal: Point3D {
                x: (2.0),
                y: (-3.0),
                z: (1.0),
            },
            distance: 3.0 / f64::sqrt(14.0),
        };

        let my_ray = Ray {
            point: Point3D {
                x: (-1.0),
                y: (4.0),
                z: (1.0),
            },
            direction: Point3D {
                x: (2.0),
                y: (-5.0),
                z: (1.0),
            },
        };
        let intersection_point = my_plane.intersect(my_ray);
        eprintln!("{}", intersection_point.unwrap()); // should be (0.6, 0, 1.8)
    }

    #[test]
    fn test_sphere_intersect() {
        let my_sphere = Sphere {
            center: Point3D { x: (0.0), y: (0.0), z: (0.0) },
            radius: 3.0,
        };
        let my_ray = Ray {
            point: Point3D { x: (2.0), y: (2.0), z: (2.0) },
            direction: Point3D { x: (-1.0), y: (-1.0), z: (-1.0) }
        };
        let intersection_point = my_sphere.intersect(my_ray).unwrap();
        eprintln!("{}", intersection_point);
        assert_eq!(intersection_point, Point3D{x: 0.0, y: 0.0, z: 3.0});

    }
}
