use vec3::Vec3;

pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new(o: Vec3, d: Vec3) -> Ray {
    Ray {
      origin: o,
      direction: d
    }
  }

  pub fn point_at_parameter(&self, t: f64) -> Vec3 {
    self.origin + self.direction * t
  }
}
