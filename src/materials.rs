use palette::LinSrgb;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Material {
    Phong(PhongModel),
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PhongModel {
    pub color: LinSrgb,
    pub k_s: f64,
    pub k_d: f64,
    pub k_a: f64,
    pub alpha: f64,
}
