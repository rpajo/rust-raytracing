use crate::vec3::Vec3;

pub fn degrees_ro_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::random(-1.0, 1.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_in_unit_sphere_normalized() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_in_unit_sphere_normalized();
    if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
        // In the same hemisphere as the normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn linear_to_gamma_color(value: f64) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    value.sqrt()
}

pub fn reflect_vector(vec: &Vec3, normal: &Vec3) -> Vec3 {
    vec - (2.0 * Vec3::dot(vec, normal) * normal)
}

pub fn refract_vector(vec: &Vec3, normal: &Vec3, refraction_ratio: f64) -> Vec3 {
    // Snell's law
    // https://en.wikipedia.org/wiki/Snell%27s_law
    let cos_theta = Vec3::dot(&-vec, normal).min(1.0);
    let ref_vec_perp = refraction_ratio * (vec + (cos_theta * normal));
    let ref_vec_para = -(1.0 - ref_vec_perp.length_squared()).abs().sqrt() * normal;
    ref_vec_perp + ref_vec_para
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Schlick's approximation
    // https://en.wikipedia.org/wiki/Schlick%27s_approximation
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
