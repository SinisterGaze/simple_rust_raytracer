use crate::math::vector::Vec3D;

use palette::LinSrgb;

pub struct LightSource {
    pub position: Vec3D,
    pub color: LinSrgb,
}
