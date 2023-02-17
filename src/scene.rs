use crate::camera::Camera;
use crate::light::LightSource;
use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::Ray};
use crate::utils;

use palette::{Clamp, ComponentWise, LinSrgb, Pixel, Srgb};
use rayon::prelude::*;
use std::sync::Arc;

pub struct Scene {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub light_sources: Vec<LightSource>,
    pub max_depth: u32,
}

impl Scene {
    pub fn get_first_intersection(&self, ray: Ray) -> Option<IntersectionData> {
        let mut best = f64::INFINITY;
        let mut winner: Option<IntersectionData> = None;
        for object in &self.objects {
            match object.intersect(ray, 0.00001, best) {
                Some(intersection_data) => {
                    best = if intersection_data.t < best {
                        intersection_data.t
                    } else {
                        best
                    };
                    winner = Some(intersection_data);
                }
                None => continue,
            }
        }
        winner
    }

    pub fn is_free_path(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        for object in &self.objects {
            match object.intersect(ray, t_min, t_max) {
                Some(_) => return false,
                None => continue,
            }
        }
        return true;
    }

    fn get_ambient_light(&self) -> LinSrgb {
        let mut sum = LinSrgb::new(0.0, 0.0, 0.0);
        for light in &self.light_sources {
            sum += light.color;
        }
        sum
    }

    pub fn trace(&self, ray: Ray, depth: u32) -> LinSrgb {
        let ambient_light = self.get_ambient_light();
        match self.get_first_intersection(ray) {
            Some(intersection) => {
                let normal = intersection.normal;
                let to_viewer = -ray.direction.unit_vector();
                let intersection_point = ray.at(intersection.t);

                let u = intersection.u;
                let v = intersection.v;
                let mut phong_color = LinSrgb::new(0.0, 0.0, 0.0); //phong_model.color;
                let phong_model = intersection.phong_data.unwrap();
                let object_color = phong_model.material.get_color_at(u, v);
                let mut shadow: bool = true;
                for light in &self.light_sources {
                    let point_to_light = light.position - intersection_point;
                    let dist = point_to_light.norm();
                    let to_light = Ray {
                        origin: intersection_point + 0.0001 * normal,
                        direction: point_to_light / dist,
                    };
                    let dot_diffuse = (normal * to_light.direction) as f32;
                    if dot_diffuse > 0.0 && self.is_free_path(to_light, 0.0, dist) {
                        shadow = false;
                        let diffuse_component = dot_diffuse;
                        let dot_specular = (-to_light.direction).reflect(normal) * to_viewer;
                        let specular_component = if dot_specular > 0.0 {
                            dot_specular.powf(phong_model.alpha as f64) as f32
                        } else {
                            0.0
                        };

                        //println!("{:?}", to_light.direction.reflect(normal) * ray_from);
                        // Add specular component to Phong Model blended color
                        phong_color = phong_color.component_wise(&light.color, |a, b| {
                            a + b * specular_component * phong_model.k_s
                        });
                        // Add diffuse component to Phong Model blended color
                        phong_color = phong_color.component_wise(&object_color, |a, b| {
                            a + b * diffuse_component * phong_model.k_d
                        });
                    }
                }
                let mut reflected_color = LinSrgb::new(0.0, 0.0, 0.0);
                if depth < self.max_depth {
                    let reflected_ray = Ray {
                        origin: intersection_point + 0.0001 * normal,
                        direction: intersection.ray.direction.reflect(normal),
                    };
                    reflected_color = self.trace(reflected_ray, depth + 1);
                }
                let ambient_color = object_color
                    .component_wise(&ambient_light, |a, b| 0.05 * phong_model.k_a * (a + b));

                reflected_color = if shadow {
                    reflected_color.component_wise_self(|a| phong_model.k_s * phong_model.k_a * a)
                } else {
                    reflected_color.component_wise_self(|a| phong_model.k_s * a)
                };

                let final_color = phong_color + reflected_color + ambient_color;
                return final_color.clamp();
            }
            None => LinSrgb::new(0.0, 0.0, 0.0), //LinSrgb::new(117.0 / 255.0, 186.0 / 255.0, 1.0),
        }
    }
}

pub struct Renderer {
    pub scene: Scene,
    pub camera: Camera,
    pub width: u32,
    pub height: u32,
    pub h_fov: f64,
}

impl Renderer {
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
                Srgb::from_linear(self.scene.trace(ray, 0))
                    .into_format()
                    .into_raw::<[u8; 3]>() //<---- convert into byte array
            })
            .flatten()
            .collect()
    }

    pub fn render_scene(&self) -> Vec<u8> {
        let mut total = 0;
        (0..self.height)
            .into_par_iter()
            .map(|y| {
                self.render_row(y)
            })
            .flatten()
            .collect()
    }

    fn get_inv_aspect_ratio(&self) -> f64 {
        ((self.height - 1) as f64) / ((self.width - 1) as f64)
    }

    pub fn save_ppm(&self, filename: &str) -> Result<(), std::io::Error> {
        utils::save_ppm(filename, self.width, self.height, &self.render_scene())
    }
}
