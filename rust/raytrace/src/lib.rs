pub mod vec3;
pub mod ray;
pub mod hitable;
pub mod material;
pub mod camera;
pub mod aabb;
pub mod scenes;
pub mod renderer;
pub mod rt_rand;

#[cfg(test)]
mod tests {

  use vec3::Vec3;
  use ray::Ray;
  use rt_rand::*;

  #[test]
  fn test_pt_at_param() {
    let r = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), 0.0);
    let p = r.point_at_parameter(0.5);
    let expected = Vec3::new(1.0, 1.5, 1.0);
    assert_eq!(expected, p);
  }

  #[test]
  fn test_ops() {
    assert_eq!(Vec3::new(0.0, 1.0, 0.0).length(), 1.0);
    assert_eq!(Vec3::new(10.0, 0.0, 0.0).normalized(), Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(Vec3::cross(&Vec3::new(0.0, 0.0, 1.0), &Vec3::new(0.0, 1.0, 0.0)), Vec3::new(-1.0, 0.0, 0.0));

    let f = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(f[0], 1.0);
    assert_eq!(f[1], 2.0);
    assert_eq!(f[2], 3.0);
  }
}

