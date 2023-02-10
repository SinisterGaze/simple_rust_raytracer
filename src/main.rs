#[macro_use]
extern crate impl_ops;
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

impl Point3D {
    fn norm(&self) -> f64 {
        f64::sqrt(self * self)
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

pub struct Plane {
    normal: Point3D,
    distance: f64,
}

#[cfg(test)]
mod tests {
    use real_float::IsFinite;

    use crate::Point3D;

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
}
