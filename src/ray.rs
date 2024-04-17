use crate::{
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
            ray_color: Color3::from_f64(0.5, 0.7, 1.0),
        }
    }

    pub fn cast(&self, scalar: f64) -> Pos3 {
        self.pos + (scalar * self.dir)
    }

    pub fn ray_color(&self, world: &World) -> Color3 {
        let hit = world.hit_objects(self, 0.0, f64::MAX);
        if let Some(hit) = hit {
            let normal_vec = &hit.normal;
            return Color3::from_f64(normal_vec.x + 1.0, normal_vec.y + 1.0, normal_vec.z + 1.0)
                * 0.5;
        }
        let dir_normalized = self.dir.normalize();
        let y_ratio = 0.5 * (dir_normalized.y + 1.0); // move normalized y-axis from [-1, 1] to [0, 2] and multiply with .5 for [0, 1]

        (1.0 - y_ratio) * Color3::WHITE + y_ratio * self.ray_color
    }

    pub fn hit_sphere(&self, sphere_pos: &Pos3, radius: f64) -> Option<f64> {
        let ray_to_sphere = sphere_pos - self.pos;
        let a = self.dir.length_squared();
        let h = Vec3::dot(&self.dir, &ray_to_sphere);
        // let b = -2.0 * Vec3::dot(&self.dir, &ray_to_sphere);
        let c = ray_to_sphere.length_squared() - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        Some((h - f64::sqrt(discriminant)) / a)
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
        let color_up = ray_up.ray_color(&world);
        let color_down = ray_down.ray_color(&world);

        assert_eq!(color_up, ray_up.ray_color);
        assert_eq!(color_down, Color3::new(1.0, 1.0, 1.0));
    }
}
