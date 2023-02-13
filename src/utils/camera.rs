use crate::math::vector::Vec3D;

pub struct Camera {
    pub origin: Vec3D,
    pub look_at: Vec3D,
    pub up: Vec3D,
}
