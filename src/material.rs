use rand::Rng;

use crate::{
    objects::object::HitRecord,
    ray::Ray,
    utils::helpers::{
        random_in_unit_sphere_normalized, reflect_vector, reflectance, refract_vector,
    },
    vec3::{Color3, Vec3},
};

pub trait Material {
    fn reflect(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color3, Ray)>;
}
pub struct Lambert {
    albedo: Color3,
}
pub struct Metallic {
    albedo: Color3,
    fuzz: f64,
}

pub struct Dielectric {
    refraction_index: f64,
}
pub struct NormalMaterial {}

impl Lambert {
    pub fn new(color: Color3) -> Self {
        Self { albedo: color }
    }
}
impl Material for Lambert {
    fn reflect(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Color3, Ray)> {
        let mut scattered_dir = hit.normal + random_in_unit_sphere_normalized();
        if scattered_dir.near_zero() {
            scattered_dir = hit.normal;
        }
        let scattered_ray = Ray::new(hit.point, scattered_dir);
        Some((self.albedo, scattered_ray))
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Metallic {
    pub fn default() -> Self {
        Metallic {
            albedo: Color3::WHITE,
            fuzz: 0.0,
        }
    }
    pub fn new(color: Color3, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz,
        }
    }
}

impl Material for Metallic {
    fn reflect(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = reflect_vector(&ray.dir, &hit.normal);
        let reflected_fuzzed =
            reflected.normalize() + self.fuzz * random_in_unit_sphere_normalized();

        let has_same_direction = Vec3::dot(&reflected_fuzzed, &hit.normal) > 0.0;
        if !has_same_direction {
            return None;
        }
        let reflected_ray = Ray::new(hit.point, reflected_fuzzed);

        Some((self.albedo, reflected_ray))
    }
}

impl Material for NormalMaterial {
    fn reflect(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Color3, Ray)> {
        let color = Color3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5;

        // todo: probably better as post process?
        Some((color, Ray::new(hit.point, Vec3::ZERO)))
    }
}

impl Material for Dielectric {
    fn reflect(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color3, Ray)> {
        let color = Color3::WHITE;
        let reflection_index = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = &ray.dir.normalize();

        let cos_theta = Vec3::dot(&-unit_dir, &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = reflection_index * sin_theta > 1.0;
        // only calculate reflectance if not yet reflected
        let perfect_reflection = if !cannot_reflect {
            reflectance(cos_theta, self.refraction_index) > rand::thread_rng().gen()
        } else {
            false
        };
        let refracted_vec = if cannot_reflect || perfect_reflection {
            reflect_vector(unit_dir, &hit.normal)
        } else {
            refract_vector(unit_dir, &hit.normal, reflection_index)
        };

        Some((color, Ray::new(hit.point, refracted_vec)))
    }
}
