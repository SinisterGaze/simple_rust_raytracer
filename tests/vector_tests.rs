use simple_raytracer::math::vector::Vec3D;

#[test]
fn test_addition() {
    let a = Vec3D::new(1.0, 0.0, 1.0);
    let b = Vec3D::new(2.0, 2.0, -1.0);
    assert_eq!(a + b, Vec3D::new(3.0, 2.0, 0.0));
}

#[test]
fn test_subtraction() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(1.0, -1.0, 1.0);
    assert_eq!(a - b, Vec3D::new(0.0, 3.0, 2.0));
}

#[test]
fn test_scalar_mult() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = a * 5.0;
    let c = 5.0 * a;
    assert_eq!(b, c);
    assert_eq!(b, Vec3D::new(5.0, 10.0, 15.0));
}

#[test]
fn test_scalar_div() {
    let a = Vec3D::new(2.0, 4.0, 6.0);
    let b = a / 2.0;
    assert_eq!(b, Vec3D::new(1.0, 2.0, 3.0));
    assert!(!(a / 0.0).is_finite());
    let zero = Vec3D::default();
    assert!(!(zero / 0.0).is_finite());
}

#[test]
fn test_dot_product() {
    let a = Vec3D::new(2.0, 4.0, 6.0);
    let b = Vec3D::new(1.0, -1.0, 1.0);
    assert_eq!(a * b, b * a);
    assert_eq!(a * b, 4.0);
}

#[test]
fn test_cross_product() {
    let i = Vec3D::new(1.0, 0.0, 0.0);
    let j = Vec3D::new(0.0, 1.0, 0.0);
    let k = Vec3D::new(0.0, 0.0, 1.0);
    assert_eq!(Vec3D::cross(i, j), k);
    assert_eq!(Vec3D::cross(j, k), i);
    assert_eq!(Vec3D::cross(k, i), j);

    assert_eq!(Vec3D::cross(j, i), -k);
    assert_eq!(Vec3D::cross(k, j), -i);
    assert_eq!(Vec3D::cross(i, k), -j);
}
#[test]
fn test_norm() {
    let mut a = Vec3D::new(1.0, 2.0, 2.0);
    assert_eq!(a.norm(), 3.0); // sqrt(1*1 + 2*2 + 2*2) = sqrt(9) = 3
    let b = a.unit_vector();
    a.normalize();
    assert_eq!(b.norm(), 1.0); // norm of normalized vector is a unit vector
    assert_eq!(a.norm(), 1.0);
    let zero = Vec3D::default();
    assert_eq!(zero.norm(), 0.0); // norm of zero vector is zero
}
