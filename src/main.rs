use camera::Camera;
use objects::sphere::Sphere;

use crate::vec3::Vec3;
use crate::world::World;

mod camera;
mod image;
mod objects;
mod ray;
mod utils;
mod vec3;
mod world;

fn main() -> std::io::Result<()> {
    let filename = "render.ppm";
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1024;
    let camera = Camera::new(image_width, aspect_ratio);

    let mut world = World::new();
    world.add_object(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add_object(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    match camera.render(&world, filename) {
        Ok(_) => println!("Rendering finished"),
        Err(_) => eprintln!("Failed to render image"),
    }

    Ok(())
}
