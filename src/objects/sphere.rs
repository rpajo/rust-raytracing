use std::rc::Rc;

use super::object::{HitRecord, Object};
use crate::{
    material::Material,
    ray::Ray,
    utils::interval::Interval,
    vec3::{Pos3, Vec3},
};

pub struct Sphere {
    center: Pos3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<HitRecord> {
        let ray_to_sphere = self.center - ray.pos;
        let a = ray.dir.length_squared();
        let h = Vec3::dot(&ray.dir, &ray_to_sphere);
        // let b = -2.0 * Vec3::dot(&self.dir, &ray_to_sphere);
        let c = ray_to_sphere.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let disc_sqr = discriminant.sqrt();
        let mut root: f64 = (h - disc_sqr) / a;

        // try first root
        if !t_interval.contains_including(root) {
            // try second root
            root = (h + disc_sqr) / a;
            if !t_interval.contains_including(root) {
                return None;
            }
        }

        let hit_point = ray.cast(root);
        let normal = (hit_point - self.center) / self.radius;
        let mut hit_record = HitRecord {
            ray_scalar: root,
            point: hit_point,
            normal,
            front_face: Vec3::dot(&ray.dir, &normal) > 0.0,
            material: self.material.clone(),
        };
        hit_record.set_face_normal(ray, &normal);

        Some(hit_record)
    }
}

impl Sphere {
    // todo: fix lifetime
    pub fn new(position: Vec3, radius: f64, material: impl Material + 'static) -> Self {
        Self {
            center: position,
            radius,
            material: Rc::new(material),
        }
    }
}
