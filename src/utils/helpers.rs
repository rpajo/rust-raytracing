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
