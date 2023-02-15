use crate::camera::Camera;
use crate::light::LightSource;
use crate::materials::Material;
use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::Ray};
use crate::utils;

use palette::{Clamp, ComponentWise, LinSrgb, Pixel, Srgb, LinLuma};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
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
        match self.get_first_intersection(ray) {
            Some(intersection) => {
                let normal = intersection.normal;
                let ray_from = -intersection.ray.direction;
                let intersection_point = intersection.ray.at(intersection.t);
                match intersection.material {
                    Material::Phong(phong_model) => {
                        let mut object_color = LinSrgb::new(0.0, 0.0, 0.0);
                        for light in &self.light_sources {
                            let point_to_light = light.position - intersection_point;
                            let dist = point_to_light.norm();
                            let light_ray = Ray {
                                origin: intersection_point + 0.001 * normal,
                                direction: point_to_light / dist,
                            };
                            let cos_theta = normal * light_ray.direction;
                            if cos_theta > 0.0 && self.is_free_path(light_ray, 0.0, dist) {
                                let diffuse_component =
                                    (phong_model.k_d * (light_ray.direction * normal)) as f32;
                                let specular_component = (phong_model.k_s
                                    * (light_ray.direction.reflect(normal).unit_vector()
                                        * ray_from)
                                        .powf(phong_model.alpha))
                                    as f32;
                                //println!("{:?} {:?}", diffuse_component, specular_component);
                                object_color = object_color
                                    .component_wise(&light.color, |a, b| {
                                        a + b * (diffuse_component + specular_component) // / ((dist*dist) as f32)
                                    })
                                    .clamp();
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
                        return object_color + reflected_color.component_wise_self(|x| x);
                    }
                    Material::None => return LinSrgb::new(0.0, 0.0, 0.0),
                }
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
        (0..self.height)
            .into_iter()
            .map(|y| self.render_row(y))
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
