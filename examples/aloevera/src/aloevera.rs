use simple_raytracer::camera::Camera;
use simple_raytracer::light::LightSource;
use simple_raytracer::materials::*;
use simple_raytracer::math::vector::Vec3D;
use simple_raytracer::objects::mesh::Mesh;
use simple_raytracer::objects::plane::Plane;
use simple_raytracer::scene::*;

use image;
use palette::LinSrgb;
use std::sync::Arc;

fn main() {
    let diffuse_grey = PhongModel {
        material: Material::Color(LinSrgb::new(1.0, 1.0, 1.0)),
        k_s: 0.0,
        k_d: 0.1,
        k_a: 0.9,
        alpha: 700.0,
    };
    let mut aloevera = Mesh::from_file("assets/objects/aloevera.obj").unwrap();
    aloevera.set_phong_data(diffuse_grey);
    let light = LightSource {
        position: Vec3D::new(3.0, 100.0, -30.0),
        color: LinSrgb::new(1.0, 1.0, 1.0),
    };
    let floor = Plane {
        normal: Vec3D::new(0.0, 1.0, 0.0),
        distance: 0.0,
        phong_data: Some(PhongModel {
            material: Material::Color(LinSrgb::new(1.0, 1.0, 1.0)),
            k_s: (0.5),
            k_d: (0.5),
            k_a: (0.02),
            alpha: (500.0),
        }),
    };
    let my_scene = Scene {
        objects: vec![Arc::new(aloevera), Arc::new(floor)],
        light_sources: vec![light],
        max_depth: 1,
    };
    let my_camera = Camera {
        origin: Vec3D::new(0.0, 0.25, -0.5),
        look_at: Vec3D::new(0.0, 0.25, 0.0),
        up: Vec3D::new(0.0, 1.0, 0.0),
    };
    let renderer = Renderer {
        scene: my_scene,
        width: 800,
        height: 600,
        camera: my_camera,
        h_fov: f64::to_radians(90.0),
    };
    // Capture scene as pixel array
    let pixels = renderer.render_scene();

    let path = "examples/aloevera/output";
    let name = "aloevera";

    // Save pixel array as png
    image::RgbImage::from_vec(renderer.width, renderer.height, pixels)
        .unwrap()
        .save(format!("{path}/{name}.png").as_str())
        .expect(format!("Could not create file {name}.png").as_str());
}
