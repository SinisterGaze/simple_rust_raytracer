use crate::math::vector::Vec3D;
use crate::objects::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vert_a: Vec3D,
    pub vert_b: Vec3D,
    pub vert_c: Vec3D,
    pub normal: Option<[Vec3D; 3]>, // 3 vertex normals for mesh shading
    pub uv: Option<[(f64, f64); 3]>,
}

impl Triangle {
    pub fn get_normal_at(&self, point: Vec3D) -> Vec3D {
        if let Some(vertex_normals) = self.normal {
            let (u, v) = self.point_to_uv(point);
            u * vertex_normals[0] + v * vertex_normals[1] + (1.0 - u - v) * vertex_normals[2]
        } else {
            self.get_plane_normal()
        }
    }

    pub fn get_plane_normal(&self) -> Vec3D {
        let ab = self.vert_b - self.vert_a;
        let ac = self.vert_c - self.vert_a;
        Vec3D::cross(ab, ac).unit_vector()
    }

    pub fn get_intersection(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let normal = self.get_plane_normal();
        let distance = self.vert_a * normal;
        if (normal * ray.direction).abs() <= f64::EPSILON {
            None
        } else {
            let t: f64 = (distance - ray.origin * normal) / (ray.direction * normal);
            if t_min < t && t < t_max {
                let p = ray.at(t);

                let test1: bool =
                    Vec3D::cross(self.vert_b - self.vert_a, p - self.vert_a) * normal >= 0.0;
                let test2: bool =
                    Vec3D::cross(self.vert_c - self.vert_b, p - self.vert_b) * normal >= 0.0;
                let test3: bool =
                    Vec3D::cross(self.vert_a - self.vert_c, p - self.vert_c) * normal >= 0.0;
                if test1 && test2 && test3 {
                    Some(t)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub fn point_to_uv(&self, point: Vec3D) -> (f64, f64) {
        let ab = self.vert_b - self.vert_a;
        let ac = self.vert_c - self.vert_a;
        let ap = point - self.vert_a;
        let den = ab.norm2() * ac.norm2() - (ab * ac) * (ab * ac);
        let v = (ac.norm2() * (ap * ab) - (ab * ac) * (ap * ac)) / den;
        let w = (ab.norm2() * (ap * ac) - (ab * ac) * (ap * ab)) / den;
        let u = 1.0 - v - w;
        (u, v)
    }

    pub fn min_z(&self) -> f64 {
        [self.vert_a.z, self.vert_b.z, self.vert_c.z]
            .into_iter()
            .reduce(|a, b| f64::min(a, b))
            .unwrap()
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.min_z() == other.min_z()
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.min_z().partial_cmp(&other.min_z())
    }
}
