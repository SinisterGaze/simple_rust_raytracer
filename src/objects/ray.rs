use crate::math::vector::Vec3D;
pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3D {
        self.origin + t * self.direction
    }
}