use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { x, y, z }
    }

    pub fn from_vec(coords: Vec<f64>) -> Self {
        assert_eq!(coords.len(), 3);
        Vec3D {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
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

    pub fn cross(self, b: Vec3D) -> Self {
        Vec3D {
            x: (self.y * b.z - self.z * b.y),
            y: -(self.x * b.z - self.z * b.x),
            z: (self.x * b.y - self.y * b.x),
        }
    }
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn almost_zero(self) -> bool {
        self.norm2() <= f64::EPSILON
    }

    // projects self onto other, i.e. returns the component of self that is parallel to other
    pub fn project_onto(self, other: Vec3D) -> Self {
        let scalar = (other * self) / other.norm2();
        scalar * other
    }
    // returns self's perpendicular component relative to other
    pub fn perp(self, other: Vec3D) -> Self {
        self - self.project_onto(other)
    }
    // reflects incoming vector self on plane given by normal vector
    pub fn reflect(self, normal: Vec3D) -> Self {
        self - 2.0 * self.project_onto(normal)
    }

    pub fn almost_equal(self, other: Vec3D) -> bool {
        (self - other).almost_zero()
    }
}

impl std::fmt::Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
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
