use simple_raytracer::camera::Camera;
use simple_raytracer::math::vector::Vec3D;
use simple_raytracer::objects::{plane::Plane, sphere::Sphere};
use simple_raytracer::scene::*;

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
    let renderer = Renderer {
        scene: my_scene,
        width: 1920,
        height: 1080,
        camera: my_camera,
        h_fov: f64::to_radians(90.0),
    };
    // Capture scene as pixel array
    let pixels = renderer.render_scene();

    let path = "examples/two_balls";
    let name = "two_balls";

    // Save pixel array as ppm
    use simple_raytracer::utils::save_ppm;
    save_ppm(
        format!("{path}/{name}.ppm").as_str(),
        renderer.width,
        renderer.height,
        &pixels,
    )
    .expect(format!("Could not create file {name}.ppm").as_str());

    // Save pixel array as png
    image::RgbImage::from_vec(renderer.width, renderer.height, pixels)
        .unwrap()
        .save(format!("{path}/{name}.png").as_str())
        .expect(format!("Could not create file {name}.png").as_str());
}