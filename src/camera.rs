use rand::Rng;
use std::{
    fs::OpenOptions,
    io::{Error, Write},
};

use crate::{
    image::color_to_ppm,
    ray::Ray,
    utils::helpers::degrees_ro_radians,
    vec3::{Color3, Pos3, Vec3},
    world::World,
};

pub struct Camera {
    pub position: Vec3,
    pub look_at: Pos3,

    pub aspect_ratio: f64,
    pub render_image_width: i32,
    pub anti_aliasing: AntiAliasingMethod,
    pub max_ray_bounces: u16,

    render_image_heigh: i32,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00_loc: Vec3,
}

pub struct CameraSetup {
    pub image_width: i32,
    pub aspect_ratio: f64,
    pub vfow_deg: f64,
    pub position: Vec3,
    pub look_at: Vec3,
    pub anti_aliasing: AntiAliasingMethod,
    pub max_ray_bounces: u16,
}

pub enum AntiAliasingMethod {
    None,
    UniformSuperSampling(i8),
    RandomSuperSampling(i8),
}

impl Camera {
    pub fn new(config: CameraSetup) -> Self {
        let image_width = config.image_width;
        let image_height = (image_width as f64 / config.aspect_ratio) as i32;
        let image_ratio = image_width as f64 / image_height as f64;

        let focal_length = (config.position - config.look_at).length();

        let theta = degrees_ro_radians(config.vfow_deg);
        let h: f64 = f64::tan(theta / 2.0);
        let vp_height = 2.0 * h * focal_length;
        let vp_width = vp_height * image_ratio;

        let up_vector = Vec3::new(0.0, 1.0, 0.0);
        let camera_position = config.position;
        let camera_w = (config.position - config.look_at).normalize();
        let camera_u = Vec3::cross(&up_vector, &camera_w).normalize();
        let camera_v = Vec3::cross(&camera_w, &camera_u);
        // view port relative vectors (x along the width and y along the height of the viewport)

        let vp_u_dir = vp_width * camera_u;
        let vp_v_dir = vp_height * -camera_v;
        let vp_origin_upper_left =
            camera_position - (focal_length * camera_w) - (vp_u_dir / 2.0) - (vp_v_dir / 2.0);

        let pixel_delta_u = vp_u_dir / image_width as f64;
        let pixel_delta_v = vp_v_dir / image_height as f64;
        let pixel_00_loc = vp_origin_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            position: camera_position,
            look_at: config.look_at,
            aspect_ratio: config.aspect_ratio,
            render_image_width: config.image_width,
            render_image_heigh: image_height,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            anti_aliasing: config.anti_aliasing,
            max_ray_bounces: config.max_ray_bounces,
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
                        ray.ray_color(world, self.max_ray_bounces)
                    }
                    AntiAliasingMethod::RandomSuperSampling(samples) => {
                        let mut color_sum = Color3::ZERO;
                        for _ in 0..samples {
                            let ray = self.get_random_ray(x, y);
                            let ray_color = ray.ray_color(world, self.max_ray_bounces);
                            color_sum += ray_color;
                        }
                        color_sum / samples as f64
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
