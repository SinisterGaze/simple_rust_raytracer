use crate::materials::PhongModel;
use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::*, triangle::Triangle};

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub phong_data: Option<PhongModel>,
}

impl Mesh {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();
        let mut vbo: Vec<Vec3D> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();
        for line in lines {
            if let Ok(data) = line {
                if data.len() > 0 {
                    match &data[..1] {
                        "v" => {
                            let vertex = Vec3D::from_vec(
                                data[1..]
                                    .split(" ")
                                    .filter(|x| !x.is_empty())
                                    .map(|n| n.parse::<f64>().unwrap())
                                    .collect(),
                            );
                            vbo.push(vertex);
                        }
                        "f" => {
                            let face: Vec<usize> = data[1..]
                                .split(" ")
                                .filter(|x| !x.is_empty())
                                .map(|n| n.parse::<usize>().unwrap() - 1)
                                .collect();
                            assert_eq!(face.len(), 3);
                            let triangle = Triangle {
                                vert_a: vbo[face[0]],
                                vert_b: vbo[face[1]],
                                vert_c: vbo[face[2]],
                            };
                            triangles.push(triangle);
                        }
                        _ => (),
                    }
                }
            }
        }
        triangles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(Mesh {
            triangles: triangles,
            phong_data: None,
        })
    }

    pub fn set_phong_data(&mut self, phong_data: PhongModel) {
        self.phong_data = Some(phong_data);
    }
}

impl Hittable for Mesh {
    /*fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
        let mut best = t_max;
        let mut winner: Option<Triangle> = None;
        for triangle in &self.triangles {
            if let Some(t) = triangle.get_intersection(ray, t_min, best) {
                best = t;
                winner = Some(*triangle);
            }
        }
        if let Some(triangle) = winner {
            let (u, v) = (0.0, 0.0); // TODO! triangle.point_to_uv(ray.at(best));
            Some(IntersectionData {
                ray: ray,
                t: best,
                normal: triangle.normal(),
                phong_data: self.phong_data.as_ref(),
                u: u,
                v: v,
            })
        } else {
            None
        }
    }*/
     
    fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
        for triangle in &self.triangles {
            if let Some(t) = triangle.get_intersection(ray, t_min, t_max) {
                let (u, v) = (0.0, 0.0); // TODO! triangle.point_to_uv(ray.at(best));
                return Some(IntersectionData {
                    ray: ray,
                    t: t,
                    normal: triangle.normal(),
                    phong_data: self.phong_data.as_ref(),
                    u: u,
                    v: v,
                });
            }
        }
        None
    }

    fn get_phong_data(&self) -> Option<&PhongModel> {
        self.phong_data.as_ref()
    }

    #[allow(unused)]
    fn point_to_uv(&self, point: Vec3D) -> (f64, f64) {
        (0.0, 0.0)
    }
}
