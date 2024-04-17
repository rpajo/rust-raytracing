use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::world::World;
use crate::{image::color_to_ppm, objects::sphere::Sphere};
use std::{fs::OpenOptions, io::Write};

mod image;
mod objects;
mod ray;
mod utils;
mod vec3;
mod world;

fn main() -> std::io::Result<()> {
    let filename = "render.ppm";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    let mut write_buffer = String::new();

    let image_aspect_ratio = 16.0 / 9.0;
    let render_image_width = 1024;
    let render_image_height = (render_image_width as f32 / image_aspect_ratio) as i32;

    let viewport_height = 2.0;
    let image_ratio = render_image_width as f32 / render_image_height as f32;
    let viewport_width = image_ratio * viewport_height; // use `image_ratio` for exact ratio
    let focal_length = 1.0;

    let camera_pos = Vec3::ZERO;

    // view port relative vectors (x along the width and y along the height of the viewport)
    // u => x axis in our basic case
    // v => y axis in our basic case
    // todo: if camera is pointing somewhere else, these vectors must also change
    let vp_u_dir = Vec3::new(viewport_width, 0.0, 0.0);
    let vp_v_dir = Vec3::new(0.0, -viewport_height, 0.0);
    let vp_origin_upper_left =
        camera_pos - (0.5 * vp_u_dir) - (0.5 * vp_v_dir) - Vec3::new(0.0, 0.0, focal_length);

    let pixel_delta_u = vp_u_dir / render_image_width as f32;
    let pixel_delta_v = vp_v_dir / render_image_height as f32;
    let pixel_00_loc = vp_origin_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut world = World::new();
    world.add_object(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add_object(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    // Append ppm header
    write_buffer
        .push_str(format!("P3\n{} {}\n255\n", render_image_width, render_image_height).as_str());

    for y in 0..render_image_height {
        // eprintln!("Scan-lines processed: {}/{}", y, render_image_height);

        for x in 0..render_image_width {
            let pixel_pos = pixel_00_loc + y as f32 * pixel_delta_v + x as f32 * pixel_delta_u;

            let ray = Ray::new(camera_pos, pixel_pos - camera_pos);

            let pixel_color = ray.ray_color(&world);
            write_buffer.push_str(&color_to_ppm(&pixel_color));
        }
    }

    let write_result = file.write_all(write_buffer.as_bytes());
    if write_result.is_err() {
        eprintln!("Failed to write to file");
        eprintln!("{:?}", write_result.err());
    }
    println!("Written to: {}", filename);
    Ok(())
}
