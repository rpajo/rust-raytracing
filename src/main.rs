mod camera;
mod material;
mod objects;
mod ray;
mod utils;
mod vec3;
mod world;

use crate::vec3::Vec3;
use crate::world::World;
use camera::{AntiAliasingMethod, Camera, CameraSetup};
use material::{Dielectric, Lambert, Metallic};
use objects::sphere::Sphere;
use rand::Rng;
use vec3::{Color3, Pos3};

fn main() -> std::io::Result<()> {
    let filename = "render.ppm";
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 2000;

    let camera = Camera::new(CameraSetup {
        image_width,
        aspect_ratio,
        // anti_aliasing: AntiAliasingMethod::None,
        anti_aliasing: AntiAliasingMethod::RandomSuperSampling(200),
        look_at: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        position: Vec3 {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        max_ray_bounces: 50,
        vfow_deg: 20.0,
        defocus_angle: 0.6,
        focus_distance: 10.0,
    });

    let mut world = World::new();

    let ground_mat = Lambert::new(Color3::new(0.5, 0.5, 0.5));
    world.add_object(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));
    // world.add_object(Plane::new(Vec3::new(0.0, -1.0, 0.0), ground_mat));

    let mut rng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let position = Pos3::new(
                i as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                j as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (position - Pos3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat_chance: f64 = rng.gen();

                if mat_chance < 0.7 {
                    let albedo = Color3::random(0.0, 1.0) * Color3::random(0.0, 1.0);
                    let mat = Lambert::new(albedo);
                    world.add_object(Sphere::new(position, 0.2, mat));
                } else if mat_chance < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let mat = Metallic::new(albedo, fuzz);
                    world.add_object(Sphere::new(position, 0.2, mat));
                } else {
                    let mat = Dielectric::new(1.5);
                    world.add_object(Sphere::new(position, 0.2, mat));
                }
            }
        }
    }
    let mat_1 = Dielectric::new(1.5);
    world.add_object(Sphere::new(Pos3::new(0.0, 1.0, 0.0), 1.0, mat_1));
    let mat_2 = Lambert::new(Color3::new(0.4, 0.2, 0.1));
    world.add_object(Sphere::new(Pos3::new(-4.0, 1.0, 0.0), 1.0, mat_2));
    let mat_3 = Metallic::new(Color3::new(0.7, 0.6, 0.5), 0.0);
    world.add_object(Sphere::new(Pos3::new(4.0, 1.0, 0.0), 1.0, mat_3));

    match camera.render(&world, filename) {
        Ok(_) => println!("Rendering finished"),
        Err(_) => eprintln!("Failed to render image"),
    }

    Ok(())
}
