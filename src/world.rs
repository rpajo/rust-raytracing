use std::rc::Rc;

use crate::{
    objects::object::{HitRecord, Object},
    ray::Ray,
};

pub struct World {
    pub objects: Vec<Rc<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn add_object<T: 'static + Object>(&mut self, object: T) {
        self.objects.push(Rc::new(object));
    }

    pub fn hit_objects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;
        let mut nearest_hit = t_max;

        for obj in &self.objects {
            if let Some(h) = obj.hit(ray, t_min, nearest_hit) {
                nearest_hit = h.ray_scalar;
                hit = Some(h);
            }
        }

        hit
    }
}
