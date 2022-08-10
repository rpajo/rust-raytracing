use crate::vec3::Color3;

pub fn export_ppm_image(width: u32, height: u32) {
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for y in 0..height {
        // eprintln!("Scan-lines processed: {}/{}", y, height);
        for x in 0..width {
            let pixel_color = Color3::new(x as f32, (255 - y) as f32, 67 as f32);

            println!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z);
        }
    }
}
