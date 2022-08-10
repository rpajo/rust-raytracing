use crate::vec3::{Pos3, Vec3};


pub struct Ray {
  pub pos: Pos3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(position: Pos3, direction: Vec3) -> Ray {
    Ray {
      pos: position,
      dir: direction
    }
  }

  pub fn cast(&self, scalar: f32) -> Pos3 {
    self.pos + (scalar * self.dir)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn create_ray() {
    let ray = Ray::new(
      Pos3::new(1.0, 2.0, 3.0),
      Vec3::new(1.0, 0.0, 1.0),
    );

    assert_eq!(ray.cast(1.0), Pos3::new(2.0, 2.0, 4.0))
  }
}