use crate::vec3::Color3;

pub fn color_to_ppm(color: &Color3) -> String {
    format!("{} {} {}\n", color.x as i32, color.y as i32, color.z as i32)
}
