mod camera;
mod image;
mod material;
mod objects;
mod ray;
mod utils;
mod vec3;
mod world;

use crate::vec3::Vec3;
use crate::world::World;
use camera::{AntiAliasingMethod, Camera};
use material::{Dielectric, Lambert, Metallic};
use objects::sphere::Sphere;
use vec3::Color3;

fn main() -> std::io::Result<()> {
    let filename = "render.ppm";
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1024;

    let mut camera = Camera::new(image_width, aspect_ratio);
    camera.anti_aliasing = AntiAliasingMethod::RandomSuperSampling(8);
    camera.max_ray_bounces = 50;

    let mut world = World::new();

    let ground_mat = Lambert::new(Color3::new(0.8, 0.8, 0.0));
    let mat_1 = Lambert::new(Color3::new(0.1, 0.2, 0.5));
    let mat_2 = Dielectric::new(1.5);
    let mat_3 = Metallic::new(Color3::new(0.8, 0.6, 0.2), 0.3);

    world.add_object(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_mat));
    world.add_object(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, mat_1));
    world.add_object(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_2));
    world.add_object(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_3));

    match camera.render(&world, filename) {
        Ok(_) => println!("Rendering finished"),
        Err(_) => eprintln!("Failed to render image"),
    }

    Ok(())
}
