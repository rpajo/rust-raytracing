use crate::{
    ray::Ray,
    vec3::{Pos3, Vec3},
};

pub struct HitRecord {
    pub point: Pos3,
    pub normal: Vec3,
    pub ray_scalar: f32,
    pub front_face: bool,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) -> &Vec3 {
        // note: outward normal should be normalized

        self.front_face = Vec3::dot(&ray.dir, outward_normal) > 0.0;
        self.normal = if self.front_face {
            -*outward_normal
        } else {
            *outward_normal
        };

        &self.normal
    }
}