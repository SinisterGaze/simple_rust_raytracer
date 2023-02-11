use real_float;
use std::ops;

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
impl_op_ex!(*|lhs: &Vec3D, rhs: &Vec3D| -> f64 { lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z });
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
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        f64::sqrt(self * self)
    }

    pub fn norm2(&self) -> f64 {
        self * self
    }

    pub fn normalize(&mut self) {
        (*self) /= self.norm();
    }

    pub fn unit_vector(&self) -> Self {
        let mut result = self.clone();
        result.normalize();
        result
    }
    pub fn cross(a: Vec3D, b: Vec3D) -> Vec3D {
        Vec3D {
            x: (a.y * b.z - a.z * b.y),
            y: -(a.x * b.z - a.z * b.x),
            z: (a.x * b.y - a.y * b.x),
        }
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
        Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}