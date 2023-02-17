use image::io::Reader as ImageReader;
use palette::LinSrgb;
use std::error::Error;
use std::path::Path;


#[derive(Debug, Clone)]
pub enum Material {
    Color(LinSrgb),
    Texture(Texture),
    None,
}

impl Material {
    pub fn get_color_at(&self, u: f64, v: f64) -> LinSrgb {
        match self {
            Material::Color(color) => color.clone(),
            Material::Texture(texture) => texture.get_color_at(u, v),
            Material::None => LinSrgb::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhongModel {
    pub material: Material,
    pub k_s: f32,
    pub k_d: f32,
    pub k_a: f32,
    pub alpha: f32,
}

impl PhongModel {
    pub fn new() -> Self {
        PhongModel {
            material: Material::None,
            k_s: 0.0,
            k_d: 0.0,
            k_a: 0.0,
            alpha: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pixel_colors: Vec<LinSrgb>,
}

impl Texture {
    pub fn load_texture<P: AsRef<Path>>(filename: P) -> Result<Self, Box<dyn Error>> {
        let texture = ImageReader::open(filename)?.decode()?.to_rgb8();
        let w = texture.width() as usize;
        let h = texture.height() as usize;
        let buffer = texture.into_vec();
        assert_eq!((3 * w * h) as usize, buffer.len());
        let mut pixel_colors: Vec<LinSrgb> = Vec::new();
        pixel_colors.reserve((w * h) as usize);
        (0..(w * h) as usize).into_iter().for_each(|idx| {
            let r = buffer[3 * idx] as f32;
            let g = buffer[3 * idx + 1] as f32;
            let b = buffer[3 * idx + 2] as f32;
            pixel_colors.push(LinSrgb::new(r / 255.0, g / 255.0, b / 255.0));
        });
        Ok(Texture {
            width: w,
            height: h,
            pixel_colors: pixel_colors,
        })
    }

    pub fn get_color_at(&self, u: f64, v: f64) -> LinSrgb {
        let x = (u * (self.width as f64)) as usize;
        let y = (v * (self.height as f64)) as usize;
        self.pixel_colors[y * self.width + x]
    }
}
