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
use camera::{AntiAliasingMethod, Camera, CameraSetup};
use material::{Dielectric, Lambert, Metallic};
use objects::sphere::Sphere;
use vec3::Color3;

fn main() -> std::io::Result<()> {
    let filename = "render.ppm";
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1024;

    let camera = Camera::new(CameraSetup {
        image_width,
        aspect_ratio,
        anti_aliasing: AntiAliasingMethod::RandomSuperSampling(8),
        look_at: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        position: Vec3 {
            x: -2.0,
            y: 2.0,
            z: 1.0,
        },
        max_ray_bounces: 50,
        vfow_deg: 20.0,
    });

    let mut world = World::new();

    let ground_mat = Lambert::new(Color3::new(0.8, 0.8, 0.0));
    let mat_1 = Lambert::new(Color3::new(0.1, 0.2, 0.5));
    let mat_glass = Dielectric::new(1.5);
    let mat_inside_glass = Dielectric::new(1.0 / 1.5);
    let mat_3 = Metallic::new(Color3::new(0.8, 0.6, 0.2), 0.3);

    world.add_object(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_mat));
    world.add_object(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, mat_1));
    world.add_object(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_glass));
    world.add_object(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        mat_inside_glass,
    ));
    world.add_object(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_3));

    let mat_blue = Lambert::new(Color3::new(0.0, 0.0, 0.9));
    let mat_red = Lambert::new(Color3::new(0.9, 0.0, 0.0));

    let r = (std::f64::consts::PI / 4.0).cos();
    world.add_object(Sphere::new(Vec3::new(r, 0.0, -0.4), r / 4.0, mat_red));
    world.add_object(Sphere::new(Vec3::new(-r, 0.0, -0.4), r / 4.0, mat_blue));

    match camera.render(&world, filename) {
        Ok(_) => println!("Rendering finished"),
        Err(_) => eprintln!("Failed to render image"),
    }

    Ok(())
}
