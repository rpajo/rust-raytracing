use crate::vec3::Color3;

pub fn export_ppm_image(width: u32, height: u32) {
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for y in 0..height {
        // eprintln!("Scan-lines processed: {}/{}", y, height);
        for x in 0..width {
            let pixel_color = Color3::new(x as f64, (255 - y) as f64, 67 as f64);

            // println!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z);
        }
    }
}

pub fn color_to_ppm(color: &Color3) -> String {
    format!("{} {} {}\n", color.x as i32, color.y as i32, color.z as i32)
}
