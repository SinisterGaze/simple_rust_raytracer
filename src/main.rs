use simple_raytracer::math::vector::Vec3D;
use simple_raytracer::scene::*;
use simple_raytracer::objects::{sphere::Sphere, plane::Plane};
use simple_raytracer::utils::camera::Camera;

use image::{self, Rgb};

fn main() {
    let ball = Sphere {
        center: Vec3D::new(0.0, 1.0, 0.0),
        radius: 1.0,
        color: Rgb([0, 0, 255]),
    };
    let floor = Plane {
        normal: Vec3D::new(0.0, 1.0, 0.0),
        distance: 0.0,
        color: Rgb([255, 0, 0]),
    };
    let my_scene = Scene {
        objects: vec![Box::new(ball), Box::new(floor)],
        light_sources: vec![],
    };
    let my_camera = Camera {
        origin: Vec3D::new(2.0, 2.0, 2.0),
        look_at: Vec3D::new(0.0, 1.5, 0.0),
        up: Vec3D::new(0.0, 1.0, 0.0),
    };
    let renderer = SceneRenderer {
        scene: my_scene,
        width: 800,
        height: 600,
        camera: my_camera,
        h_fov: f64::to_radians(90.0),
    };
    let pixels = renderer.render_scene();
    let img_buffer = image::RgbImage::from_vec(renderer.width, renderer.height, pixels).unwrap();
    img_buffer.save("my_scene.png").ok();
}