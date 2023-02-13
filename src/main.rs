use simple_raytracer::math::vector::Vec3D;
use simple_raytracer::objects::{plane::Plane, sphere::Sphere};
use simple_raytracer::scene::*;
use simple_raytracer::utils::camera::Camera;

use image::{self, Rgb};

fn main() {
    let ball = Sphere {
        center: Vec3D::new(2.0, 1.0, 0.0),
        radius: 1.0,
        color: Rgb([0, 0, 255]),
    };
    let ball2 = Sphere {
        center: Vec3D::new(-2.0, 1.0, 0.0),
        radius: 1.0,
        color: Rgb([255, 0, 255]),
    };
    let floor = Plane {
        normal: Vec3D::new(0.0, 1.0, 0.0),
        distance: 0.0,
        color: Rgb([0, 0, 0]),
    };
    let my_scene = Scene {
        objects: vec![Box::new(ball), Box::new(floor), Box::new(ball2)],
        light_sources: vec![],
    };
    let my_camera = Camera {
        origin: Vec3D::new(0.0, 1.0, 5.0),
        look_at: Vec3D::new(0.0, 1.0, 0.0),
        up: Vec3D::new(0.0, 1.0, 0.0),
    };
    let renderer = SceneRenderer {
        scene: my_scene,
        width: 1920,
        height: 1080,
        camera: my_camera,
        h_fov: f64::to_radians(90.0),
    };
    // Capture scene as pixel array
    let pixels = renderer.render_scene();

    // Save pixel array as ppm
    use simple_raytracer::utils::save_ppm;
    save_ppm("my_scene.ppm", renderer.width, renderer.height, &pixels).ok();

    // Save pixel array as png
    image::RgbImage::from_vec(renderer.width, renderer.height, pixels)
        .unwrap()
        .save("my_scene.png")
        .ok();
}
