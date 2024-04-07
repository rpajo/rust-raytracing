use std::{fs::OpenOptions, io::Write};

use crate::{ray::Ray, vec3::Vec3};

mod image;
mod ray;
mod utils;
mod vec3;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("render.ppm")?;

    let mut write_buffer = String::new();

    let image_aspect_ratio = 16.0 / 9.0;
    let render_image_width = 480;
    let render_image_height = (render_image_width as f32 / image_aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = image_aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let camera_pos = Vec3::ZERO;
    // vp - viewport
    let vp_x_dir = Vec3::new(viewport_width, 0.0, 0.0);
    let vp_y_dir = Vec3::new(0.0, viewport_height, 0.0);
    let vp_origin_left_bottom =
        camera_pos - (0.5 * vp_x_dir) - (0.5 * vp_y_dir) - Vec3::new(0.0, 0.0, focal_length);

    // export_ppm_image(255, 255);

    // Append ppm header
    write_buffer
        .push_str(format!("P3\n{} {}\n255\n", render_image_width, render_image_height).as_str());

    for y in 0..render_image_height {
        eprintln!("Scan-lines processed: {}/{}", y, render_image_height);

        for x in 0..render_image_width {
            let v = (render_image_height - y) as f32 / (render_image_height - 1) as f32;
            let u = x as f32 / (render_image_width - 1) as f32;

            let ray = Ray::new(
                camera_pos,
                vp_origin_left_bottom + (u * vp_x_dir) + (v * vp_y_dir) - camera_pos,
            );

            let pixel_color = ray.ray_bg_color();
            let rounded_color_vec = format!(
                "{} {} {}\n",
                pixel_color.x as i32, pixel_color.y as i32, pixel_color.z as i32
            );
            write_buffer.push_str(rounded_color_vec.as_str());
        }
    }

    let write_result = file.write_all(write_buffer.as_bytes());
    if write_result.is_err() {
        eprintln!("Failed to write to file");
        eprintln!("{:?}", write_result.err());
    }
    Ok(())
}
