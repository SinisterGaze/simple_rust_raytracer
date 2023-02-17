use palette::LinSrgb;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Material {
    Phong(PhongModel),
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PhongModel {
    pub color: LinSrgb,
    pub k_s: f32,
    pub k_d: f32,
    pub k_a: f32,
    pub alpha: f32,
}

// TODO:
// - make the k_'s three-channeled variables (r,g,b) instead of single channel, remove color from struct
// - find good way to implement ambient light in trace method in scene.rs
