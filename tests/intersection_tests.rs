use simple_raytracer::math::vector::Vec3D;
use simple_raytracer::objects::{
    hittables::*, plane::Plane, ray::Ray, sphere::Sphere, triangle::Triangle,
};

use image::Rgb;

#[test]
fn test_plane_intersect() {
    // Simple intersection of ray onto xy-plane
    let my_plane = Plane {
        normal: Vec3D::new(0.0, 0.0, 1.0),
        distance: 0.0,
        color: Rgb([0, 0, 0]),
    };

    let my_ray = Ray {
        origin: Vec3D::new(1.0, 2.0, -3.0),
        direction: Vec3D::new(0.0, 0.0, 1.0),
    };

    let intersection_point = my_plane.intersect(&my_ray, 0.0, f64::INFINITY);
    assert_eq!(intersection_point.unwrap().t, 3.0);

    // Test of ray parallel to plane
    // No (unique) intersection point
    let parallel_ray = Ray {
        origin: Vec3D::new(1.0, 2.0, -3.0),
        direction: Vec3D::new(-5.0, 7.0, 0.0),
    };
    let intersection_point = my_plane.intersect(&parallel_ray, 0.0, f64::INFINITY);
    assert_eq!(intersection_point, None);

    // Test compared to example gathered from https://www.kristakingmath.com/blog/intersection-of-a-line-and-a-plane
    // Can't test for equality due to floating point precision, result is instead printed
    let my_plane = Plane {
        normal: Vec3D::new(2.0, -3.0, 1.0),
        distance: 3.0 / f64::sqrt(14.0),
        color: Rgb([0, 0, 0]),
    };

    let my_ray = Ray {
        origin: Vec3D::new(-1.0, 4.0, 1.0),
        direction: Vec3D::new(2.0, -5.0, 1.0),
    };
    #[allow(unused)]
    let intersection_point = my_plane.intersect(&my_ray, 0.0, f64::INFINITY);
    //eprintln!("{}", intersection_point.unwrap()); // should be (0.6, 0, 1.8)
}

#[test]
fn test_sphere_intersect() {
    let my_sphere = Sphere {
        center: Vec3D::default(),
        radius: 3.0,
        color: Rgb([0, 0, 0]),
    };
    let my_ray = Ray {
        origin: Vec3D::new(0.0, 0.0, 5.0),
        direction: Vec3D::new(0.0, 0.0, -1.0),
    };
    let intersection_point = my_sphere.intersect(&my_ray, 0.0, f64::INFINITY).unwrap();
    //eprintln!("{}", intersection_point);
    assert_eq!(intersection_point.t, 2.0);
}

#[test]
fn test_triangle_intersect() {
    let my_triangle = Triangle {
        vert_a: Vec3D::default(),
        vert_b: Vec3D::new(1.0, 0.0, 0.0),
        vert_c: Vec3D::new(0.0, 1.0, 0.0),
        color: Rgb([0, 0, 0]),
    };

    // Intersect at (0.25, 0.25, 0)
    let ray1 = Ray {
        origin: Vec3D::new(0.25, 0.25, 5.0),
        direction: Vec3D::new(0.0, 0.0, -1.0),
    };
    let p = my_triangle.intersect(&ray1, 0.0, f64::INFINITY).unwrap();
    assert_eq!(p.t, 5.0);

    // No intersection
    let ray2 = Ray {
        origin: Vec3D::new(2.0, 2.0, 5.0),
        direction: Vec3D::new(0.0, 0.0, -1.0),
    };
    let p = my_triangle.intersect(&ray2, 0.0, f64::INFINITY);
    assert_eq!(p, None);
}
