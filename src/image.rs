use crate::{utils::helpers::linear_to_gamma_color, vec3::Color3};

pub fn color_to_ppm(color: &Color3) -> String {
    // todo: fix 255 value casting rounding
    // transform from linear to gamma (gamma = 2)
    let r = (linear_to_gamma_color(color.x) * 255.0) as i32;
    let g = (linear_to_gamma_color(color.y) * 255.0) as i32;
    let b = (linear_to_gamma_color(color.z) * 255.0) as i32;
    format!("{} {} {}\n", r, g, b)
}
