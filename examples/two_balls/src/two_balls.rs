use simple_raytracer::camera::Camera;
use simple_raytracer::light::LightSource;
use simple_raytracer::materials::*;
use simple_raytracer::math::vector::Vec3D;
use simple_raytracer::objects::{plane::Plane, sphere::Sphere};
use simple_raytracer::scene::*;

use image;
use palette::LinSrgb;

fn main() {
    let ball = Sphere {
        center: Vec3D::new(2.0, 1.0, 0.0),
        radius: 1.0,
        phong_data: PhongModel {
            material: Material::Color(LinSrgb::new(1.0, 1.0, 1.0)),
            k_s: (0.7),
            k_d: (0.2),
            k_a: (0.1),
            alpha: (500.0),
        },
    };
    let ball2 = Sphere {
        center: Vec3D::new(-2.0, 1.0, 0.0),
        radius: 1.0,
        phong_data: PhongModel {
            material: Material::Color(LinSrgb::new(1.0, 0.0, 1.0)),
            k_s: (0.7),
            k_d: (0.2),
            k_a: (0.3),
            alpha: (800.0),
        },
    };
    let floor = Plane {
        normal: Vec3D::new(0.0, 1.0, 0.0),
        distance: 0.0,
        phong_data: PhongModel {
            material: Material::Color(LinSrgb::new(0.0, 0.0, 0.0)),
            k_s: (0.05),
            k_d: (0.95),
            k_a: (0.1),
            alpha: (5.0),
        },
    };
    let right_wall = Plane {
        normal: Vec3D::new(-1.0, 0.0, 0.0),
        distance: -4.0,
        phong_data: PhongModel {
            material: Material::Color(LinSrgb::new(0.3, 0.0, 0.0)),
            k_s: (0.2),
            k_d: (0.8),
            k_a: (0.1),
            alpha: (5.0),
        },
    };
    let left_wall = Plane {
        normal: Vec3D::new(1.0, 0.0, 0.0),
        distance: -4.0,
        phong_data: PhongModel {
            material: Material::Color(LinSrgb::new(0.0, 0.3, 0.0)),
            k_s: (0.2),
            k_d: (0.8),
            k_a: (0.1),
            alpha: (5.0),
        },
    };
    let back_wall = Plane {
        normal: Vec3D::new(0.0, 0.0, -1.0),
        distance: -4.0,
        phong_data: PhongModel {
            material: Material::Color(LinSrgb::new(0.0, 0.0, 0.3)),
            k_s: (0.2),
            k_d: (0.8),
            k_a: (0.1),
            alpha: (5.0),
        },
    };
    let light = LightSource {
        position: Vec3D::new(0.0, 20.0, 3.0),
        color: LinSrgb::new(1.0, 1.0, 1.0),
    };
    let my_scene = Scene {
        objects: vec![
            Box::new(ball),
            Box::new(ball2),
            Box::new(floor),
            Box::new(right_wall),
            Box::new(left_wall),
            Box::new(back_wall),
        ],
        light_sources: vec![light],
        max_depth: 3,
    };
    let my_camera = Camera {
        origin: Vec3D::new(0.0, 3.0, -5.0),
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

    let path = "examples/two_balls/output";
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
