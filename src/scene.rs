use crate::math::Vec3D;
use crate::objects::{Object3D, Ray, Plane, Sphere, Triangle};
use crate::utils::LightSource;

pub struct Scene {
    // TODO:
    // fields:
    // - container for *objects
    pub objects: Vec<Box<dyn Object3D>>,
    pub light_sources: Vec<LightSource>,
    // - container for *light_sources
    
    // public methods:
    // - get_intersections(ray, t_min, t_max) -> Vec<IntersectionData>
    //

    // private methods:
    //
    //
}

pub struct SceneRenderer {
    // TODO:
    // fields:
    // - FoV, z_near, z_far
    // - width, height
    // - camera object
    //
    // public methods: 
    // - capture(Scene object) -> &[u8]
    //
    // private methods:
    // - get_color_at(Scene object, x, y) -> Color
}