use crate::math::vector::Vec3D;
use crate::objects::{
    hittables::*, ray::Ray
};
use crate::utils::camera::Camera;
use crate::utils::light::LightSource;

use std::fs::File;
use std::io::prelude::*;

use image::{self, Rgb};

pub struct Scene {
    // TODO:
    // fields:
    // - container for *objects
    pub objects: Vec<Box<dyn Hittable>>,
    pub light_sources: Vec<LightSource>,
    // - container for *light_sources

    // public methods:
    // - get_intersections(ray, t_min, t_max) -> Vec<IntersectionData>
    //

    // private methods:
    //
    //
}

impl Scene {
    pub fn get_first_intersection(&self, ray: &Ray) -> Option<IntersectionData> {
        let mut best = f64::INFINITY;
        let mut winner: Option<IntersectionData> = None;
        for object in &self.objects {
            match object.intersect(&ray, 0.00001, best) {
                Some(intersection_data) => {
                    best = if intersection_data.t < best {
                        intersection_data.t
                    } else {
                        best
                    };
                    winner = Some(intersection_data);
                }
                _ => continue,
            }
        }
        winner
    }
    pub fn trace(&self, ray: &Ray) -> Rgb<u8> {
        match self.get_first_intersection(ray) {
            Some(intersection) => intersection.color,
            None => Rgb([255, 255, 255]),
        }
    }
}

pub struct SceneRenderer {
    // TODO:
    // fields:
    // - FoV, z_near, z_far
    // - width, height
    // - camera object
    //
    pub scene: Scene,
    pub camera: Camera,
    pub width: u32,
    pub height: u32,
    pub h_fov: f64,
}
impl SceneRenderer {
    fn render_row(&self, row: u32) -> Vec<u8> {
        let cam_dir = (self.camera.look_at - self.camera.origin).unit_vector();
        let y_dir = Vec3D::new(0.0, 1.0, 0.0);
        let cam_right = Vec3D::cross(cam_dir, y_dir).unit_vector();
        let cam_up = Vec3D::cross(cam_right, cam_dir);
        let inv_ar = self.get_inv_aspect_ratio();
        let gx = f64::tan(self.h_fov / 2.0);
        let gy = gx * inv_ar;
        let x_shift: Vec3D = ((2.0 * gx) / (self.width - 1) as f64) * cam_right;
        let y_shift: Vec3D = ((2.0 * gy) / (self.height - 1) as f64) * cam_up;
        let left_side: Vec3D = cam_dir - gx * cam_right + gy * cam_up - (row as f64) * y_shift;

        (0..self.width)
            .into_iter()
            .map(|x| {
                let ray = Ray {
                    origin: self.camera.origin,
                    direction: left_side + (x as f64) * x_shift,
                };
                self.scene.trace(&ray).0
            })
            .flatten()
            .collect()
    }
    pub fn render_scene(&self) -> Vec<u8> {
        (0..self.height)
            .into_iter()
            .map(|y| self.render_row(y))
            .flatten()
            .collect()
    }
    // ar = (w-1)/(h-1)
    fn get_inv_aspect_ratio(&self) -> f64 {
        ((self.height - 1) as f64) / ((self.width - 1) as f64)
    }
    pub fn save_ppm(&self, filename: &str) -> Result<(), std::io::Error> {
        assert_eq!(&filename[filename.len()-4..], ".ppm");
        let mut output = File::create(filename)?;
        output.write_all(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;
        let pixels = self.render_scene();
        (0..self.height).into_iter().for_each(|y| {
            (0..self.width).into_iter().for_each(|x| {
                let red = pixels[3 * (y * self.width + x) as usize];
                let green = pixels[(3 * (y * self.width + x) + 1) as usize];
                let blue = pixels[(3 * (y * self.width + x) + 2) as usize];
                output.write_all(format!("{red}").as_bytes()).ok();
                output.write_all(format!("{green}").as_bytes()).ok();
                output.write_all(format!("{blue}").as_bytes()).ok();
            });
            output.write_all("\n".as_bytes()).ok();
        });
        Ok(())
    }
}
// public methods:
// - capture(Scene object) -> &[u8]
//
// private methods:
// - get_color_at(Scene object, x, y) -> Color
