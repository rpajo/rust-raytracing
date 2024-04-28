use rand::Rng;
use std::{
    fs::OpenOptions,
    io::{Error, Write},
};

use crate::{
    ray::Ray,
    utils::helpers::{color_to_ppm, degrees_to_radians, random_in_unit_disk},
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

    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

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
    pub focus_distance: f64,
    pub defocus_angle: f64,
}

pub enum AntiAliasingMethod {
    None,
    UniformSuperSampling(u16),
    RandomSuperSampling(u16),
}

impl Camera {
    pub fn new(config: CameraSetup) -> Self {
        let image_width = config.image_width;
        let image_height = (image_width as f64 / config.aspect_ratio) as i32;
        let image_ratio = image_width as f64 / image_height as f64;

        let focus_distance = config.focus_distance;
        let defocus_angle = config.defocus_angle;

        let theta = degrees_to_radians(config.vfow_deg);
        let h: f64 = (theta / 2.0).tan();
        let vp_height = 2.0 * h * focus_distance;
        let vp_width = vp_height * image_ratio;

        let up_vector = Vec3::new(0.0, 1.0, 0.0);
        let camera_position = config.position;
        let camera_w = (config.position - config.look_at).normalize();
        let camera_u = Vec3::cross(&up_vector, &camera_w).normalize();
        let camera_v = Vec3::cross(&camera_w, &camera_u);
        // view port relative vectors (x along the width and y along the height of the viewport)

        let vp_u_dir = vp_width * camera_u;
        let vp_v_dir = vp_height * -camera_v;

        let pixel_delta_u = vp_u_dir / image_width as f64;
        let pixel_delta_v = vp_v_dir / image_height as f64;
        let viewport_upper_left =
            camera_position - (focus_distance * camera_w) - vp_u_dir / 2.0 - vp_v_dir / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            config.focus_distance * (degrees_to_radians(defocus_angle / 2.0)).tan();

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
            defocus_disk_u: camera_u * defocus_radius,
            defocus_disk_v: camera_v * defocus_radius,
            defocus_angle,
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

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.defocus_disk_sample()
        };
        let pixel_pos = self.pixel_00_loc
            + (x as f64 + offset_x) * self.pixel_delta_u
            + (y as f64 + offset_y) * self.pixel_delta_v;

        Ray::new(ray_origin, pixel_pos - ray_origin)
    }

    fn defocus_disk_sample(&self) -> Pos3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        self.position + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
