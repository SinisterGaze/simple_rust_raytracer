use crate::materials::PhongModel;
use crate::math::vector::Vec3D;
use crate::objects::{hittables::*, ray::*, triangle::Triangle};

use regex::Regex;
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
        let mut normals: Vec<Vec3D> = Vec::new();
        let mut uvs: Vec<(f64, f64)> = Vec::new();
        for line in lines {
            if let Ok(data) = line {
                if data.len() > 0 {
                    match &data[..2] {
                        "v " => {
                            let vertex = Vec3D::from_vec(
                                data[2..]
                                    .split(" ")
                                    .filter(|x| !x.is_empty())
                                    .map(|n| n.parse::<f64>().unwrap())
                                    .collect(),
                            );
                            vbo.push(vertex);
                        }
                        "f " => {
                            let mut vertices: Vec<usize> = Vec::new();
                            let mut vts: Vec<usize> = Vec::new();
                            let mut vns: Vec<usize> = Vec::new();
                            let values: Vec<&str> =
                                data[2..].split(" ").filter(|x| !x.is_empty()).collect();
                            let n_vertices = values.len();
                            for value in values.into_iter() {
                                if let Some((v, rest)) = value.split_once("/") {
                                    vertices.push(v.parse::<usize>().unwrap());
                                    if rest.len() > 0 {
                                        if let Some((vt, rest2)) = rest.split_once("/") {
                                            if vt.len() > 0 {
                                                vts.push(vt.parse::<usize>().unwrap());
                                            }
                                            if rest2.len() > 0 {
                                                vns.push(rest2.parse::<usize>().unwrap());
                                            }
                                        }
                                    }
                                } else {
                                    vertices.push(value.parse::<usize>().unwrap());
                                }
                            }
                            assert!(n_vertices >= 3);
                            (0..(n_vertices - 2)).for_each(|i| {
                                let vertex_normals = if vns.len() > 0 {
                                    Some([
                                        normals[vns[0] - 1],
                                        normals[vns[i + 1] - 1],
                                        normals[vns[i + 2] - 1],
                                    ])
                                } else {
                                    None
                                };
                                let vertex_uvs = if vts.len() > 0 {
                                    Some([
                                        uvs[vts[0] - 1],
                                        uvs[vts[i + 1] - 1],
                                        uvs[vts[i + 2] - 1],
                                    ])
                                } else {
                                    None
                                };
                                let triangle = Triangle {
                                    vert_a: vbo[vertices[0] - 1],
                                    vert_b: vbo[vertices[i + 1] - 1],
                                    vert_c: vbo[vertices[i + 2] - 1],
                                    normal: vertex_normals,
                                    uv: vertex_uvs,
                                };
                                triangles.push(triangle);
                            })
                        }
                        "vn" => {
                            let normal = Vec3D::from_vec(
                                data[2..]
                                    .split(" ")
                                    .filter(|x| !x.is_empty())
                                    .map(|n| n.parse::<f64>().unwrap())
                                    .collect(),
                            );
                            normals.push(normal);
                        }
                        "vt" => {
                            let vts: Vec<f64> = data[2..]
                                .split(" ")
                                .filter(|x| !x.is_empty())
                                .map(|n| n.parse::<f64>().unwrap())
                                .collect();
                            let uv = (vts[0], vts[1]);
                            uvs.push(uv);
                        }
                        _ => (),
                    }
                }
            }
        }
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
    fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
        let mut best = t_max;
        let mut winner: Option<Triangle> = None;
        for triangle in &self.triangles {
            if let Some(t) = triangle.get_intersection(ray, t_min, best) {
                best = t;
                winner = Some(*triangle);
            }
        }
        if let Some(triangle) = winner {
            let p = ray.at(best);
            let (u, v) = triangle.point_to_uv(p);
            Some(IntersectionData {
                ray: ray,
                t: best,
                normal: triangle.get_normal_at(p),
                phong_data: self.phong_data.as_ref(),
                u: u,
                v: v,
            })
        } else {
            None
        }
    }

    /*fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<IntersectionData> {
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
    }*/

    fn get_phong_data(&self) -> Option<&PhongModel> {
        self.phong_data.as_ref()
    }
}
