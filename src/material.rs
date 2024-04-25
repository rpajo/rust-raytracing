use crate::{
    objects::object::HitRecord,
    ray::Ray,
    utils::helpers::{random_in_unit_sphere_normalized, reflect_vector},
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
}

pub struct NormalMaterial {}

impl Lambert {
    pub fn default() -> Self {
        Self {
            albedo: Color3::WHITE,
        }
    }
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

impl Metallic {
    pub fn default() -> Self {
        Metallic {
            albedo: Color3::WHITE,
        }
    }
    pub fn new(color: Color3) -> Self {
        Self { albedo: color }
    }
}

impl Material for Metallic {
    fn reflect(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = reflect_vector(&ray.dir, &hit.normal);
        let reflected_ray = Ray::new(hit.point, reflected);
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
