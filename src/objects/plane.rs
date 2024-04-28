use std::rc::Rc;

use super::object::{HitRecord, Object};
use crate::{
    material::Material,
    ray::Ray,
    utils::interval::Interval,
    vec3::{Pos3, Vec3},
};

pub struct Plane {
    plane_up: Vec3,
    center: Pos3,
    // todo: implement plane size
    // size: f64,
    material: Rc<dyn Material>,
}

impl Object for Plane {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<HitRecord> {
        let ray_normal_dot = Vec3::dot(&ray.dir, &self.plane_up);

        // check for parallel of almost parallel cases
        if ray_normal_dot.abs() < 1e-6 {
            return None;
        }

        let p0l0 = (self.center - ray.pos) * self.plane_up;
        let scalar = Vec3::dot(&p0l0, &self.plane_up) / ray_normal_dot;

        if t_interval.contains_including(scalar) {
            // println!("Hit: {}", scalar);
            let hit_point = ray.cast(scalar);
            let mut hit_record = HitRecord {
                ray_scalar: scalar,
                point: hit_point,
                normal: self.plane_up,
                front_face: Vec3::dot(&ray.dir, &self.plane_up) > 0.0,
                material: self.material.clone(),
            };
            hit_record.set_face_normal(ray, &self.plane_up);
            Some(hit_record)
        } else {
            None
        }
    }
}

impl Plane {
    // todo: fix lifetime
    pub fn new(position: Vec3, material: impl Material + 'static) -> Self {
        Self {
            // todo: up vector to input
            plane_up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            center: position,
            material: Rc::new(material),
        }
    }
}
