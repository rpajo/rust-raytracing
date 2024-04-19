use rand::Rng;
use std::{
    fs::OpenOptions,
    io::{Error, Write},
};

use crate::{
    image::color_to_ppm,
    ray::Ray,
    vec3::{Color3, Vec3},
    world::World,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub render_image_width: i32,

    render_image_heigh: i32,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    position: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00_loc: Vec3,

    anti_aliasing: AntiAliasingMethod,
}

enum AntiAliasingMethod {
    None,
    UniformSuperSampling(i8),
    RandomSuperSampling(i8),
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_ratio = image_width as f64 / image_height as f64;

        let focal_length = 1.0;
        let vp_height = 2.0;
        let vp_width = vp_height * image_ratio;

        let camera_position = Vec3::ZERO;
        // view port relative vectors (x along the width and y along the height of the viewport)
        // u => x axis in our basic case
        // v => y axis in our basic case
        // todo: if camera is pointing somewhere else, these vectors must also change
        let vp_u_dir = Vec3::new(vp_width, 0.0, 0.0);
        let vp_v_dir = Vec3::new(0.0, -vp_height, 0.0);
        let vp_origin_upper_left = camera_position
            - (0.5 * vp_u_dir)
            - (0.5 * vp_v_dir)
            - Vec3::new(0.0, 0.0, focal_length);

        let pixel_delta_u = vp_u_dir / image_width as f64;
        let pixel_delta_v = vp_v_dir / image_height as f64;
        let pixel_00_loc = vp_origin_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            render_image_width: image_width,
            render_image_heigh: image_height,
            viewport_height: vp_height,
            viewport_width: vp_width,
            focal_length,
            position: camera_position,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            anti_aliasing: AntiAliasingMethod::RandomSuperSampling(8),
            // anti_aliasing: AntiAliasingMethod::None,
        }
    }

    pub fn render(&self, world: &World, filename: &str) -> Result<(), Error> {
        // todo: move file logic outside of the camera
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        let mut write_buffer = String::new();

        write_buffer.push_str(
            format!(
                "P3\n{} {}\n255\n",
                self.render_image_width, self.render_image_heigh
            )
            .as_str(),
        );

        for y in 0..self.render_image_heigh {
            eprintln!("Scan-lines processed: {}/{}", y, self.render_image_heigh);

            for x in 0..self.render_image_width {
                let pixel_color = match self.anti_aliasing {
                    AntiAliasingMethod::None => {
                        let pixel_pos = self.pixel_00_loc
                            + y as f64 * self.pixel_delta_v
                            + x as f64 * self.pixel_delta_u;
                        let ray = Ray::new(self.position, pixel_pos - self.position);
                        ray.ray_color(world)
                    }
                    AntiAliasingMethod::RandomSuperSampling(samples) => {
                        let mut color_sum = Color3::ZERO;
                        for _ in 0..samples {
                            let ray = self.get_random_ray(x, y);
                            let ray_color = ray.ray_color(world);
                            color_sum += ray_color;
                        }
                        Color3 {
                            x: (color_sum.x / samples as f64).round(),
                            z: (color_sum.z / samples as f64).round(),
                            y: (color_sum.y / samples as f64).round(),
                        }
                    }
                    AntiAliasingMethod::UniformSuperSampling(_samples) => Color3::ZERO,
                };

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

    fn get_random_ray(&self, x: i32, y: i32) -> Ray {
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(-0.5..0.5);
        let offset_y = rng.gen_range(-0.5..0.5);
        // println!("rand x: {}, rand y: {}", offset_x, offset_y);

        let pixel_pos = self.pixel_00_loc
            + (y as f64 + offset_x) * self.pixel_delta_v
            + (x as f64 + offset_y) * self.pixel_delta_u;

        Ray::new(self.position, pixel_pos - self.position)
    }
}
