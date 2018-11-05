use vec3::Vec3;
use ray::Ray;
use std::f64::consts::PI;
use rt_rand::*;

pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  u: Vec3,
  v: Vec3,
  // w: Vec3,
  lens_radius: f64,
  time0: f64,
  time1: f64,
}

impl Camera {
  pub fn new(look_from: &Vec3, look_at: &Vec3, v_up: &Vec3, vfov: f64, aspect: f64, aperature: f64, focus_dist: f64, time0: f64, time1: f64) -> Camera {
    let theta = vfov * PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = half_height * aspect;

    let w = (*look_from - *look_at).normalized();
    let u = (Vec3::cross(v_up, &w)).normalized();
    let v = Vec3::cross(&w, &u).normalized();

    Camera {
      lower_left_corner: *look_from - half_width * u * focus_dist - half_height * v * focus_dist - w * focus_dist,
      horizontal: 2.0 * half_width * u * focus_dist,
      vertical: 2.0 * half_height * v * focus_dist,
      origin: look_from.clone(),
      u,
      v,
      // w,
      lens_radius: aperature / 2.0,
      time0,
      time1
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.x + self.v * rd.y;
    let time = self.time0 + (self.time1-self.time0)*rand_f64();
    Ray::new(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset, time)
  }
}


