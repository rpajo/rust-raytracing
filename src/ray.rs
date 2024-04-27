use crate::{
    utils::interval::Interval,
    vec3::{Color3, Pos3, Vec3},
    world::World,
};

pub struct Ray {
    pub pos: Pos3,
    pub dir: Vec3,
    pub ray_color: Color3,
}

impl Ray {
    pub fn new(position: Pos3, direction: Vec3) -> Ray {
        Ray {
            pos: position,
            dir: direction,
            ray_color: Color3::new(0.5, 0.7, 1.0),
        }
    }

    pub fn cast(&self, scalar: f64) -> Pos3 {
        self.pos + (scalar * self.dir)
    }

    pub fn ray_color(&self, world: &World, bounces_remaining: u16) -> Color3 {
        if bounces_remaining == 0 {
            // println!("Max bounces exceeded");
            return Color3::BLACK;
        }

        let hit = world.hit_objects(self, &Interval::new(0.000001, f64::MAX));
        if let Some(hit) = hit {
            return match hit.material.reflect(&self, &hit) {
                Some((attenuation, reflected_ray)) => {
                    attenuation * reflected_ray.ray_color(world, bounces_remaining - 1)
                }
                None => Color3::BLACK,
            };
        }
        let dir_normalized = self.dir.normalize();
        let y_ratio = 0.5 * (dir_normalized.y + 1.0); // move normalized y-axis from [-1, 1] to [0, 2] and multiply with .5 for [0, 1]

        (1.0 - y_ratio) * Color3::WHITE + y_ratio * self.ray_color
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_ray() {
        let ray = Ray::new(Pos3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 0.0, 1.0));

        assert_eq!(ray.cast(1.0), Pos3::new(2.0, 2.0, 4.0))
    }

    #[test]
    fn default_ray_color() {
        let mut ray_up = Ray::new(Pos3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0));
        let ray_down = Ray::new(Pos3::new(1.0, 2.0, 3.0), Vec3::new(0.0, -1.0, 0.0));
        ray_up.ray_color = Color3::new(1.0, 0.0, 0.0);

        let world = World::new();
        let color_up = ray_up.ray_color(&world, 1);
        let color_down = ray_down.ray_color(&world, 1);

        assert_eq!(color_up, ray_up.ray_color);
        assert_eq!(color_down, Color3::new(1.0, 1.0, 1.0));
    }
}
