use std::sync::Arc;

use hitable::{Hitable, HitablePtr, HitRecord};
use material::{Material, MaterialPtr, ScatterInfo};
use texture::TexturePtr;
use rt_rand::*;
use ray::Ray;
use aabb::Aabb;
use vec3::Vec3;

pub struct Isotropic {
  albedo: TexturePtr,
}

impl Isotropic {
  pub fn new(albedo: TexturePtr) -> Isotropic {
    Isotropic {
      albedo: Arc::clone(&albedo)
    }
  }

  pub fn rc(albedo: TexturePtr) -> Arc<Isotropic> {
    Arc::new(Isotropic::new(albedo))
  }
}

impl Material for Isotropic {
  fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo> {
    let scattered = Ray::new(hit.p, random_in_unit_sphere(), ray.time);
    Some(ScatterInfo {
      attenuation: self.albedo.value(hit.u, hit.v, &hit.p),
      scattered
    })
  }
}

pub struct ConstantMedium {
  boundary: HitablePtr,
  density: f64,
  material: MaterialPtr
}

impl ConstantMedium {
  pub fn new(boundary: &HitablePtr, density: f64, phase_texture: TexturePtr) -> ConstantMedium {
    let material: MaterialPtr = Isotropic::rc(phase_texture);
    ConstantMedium {
      boundary: Arc::clone(boundary),
      density,
      material
    }
  }

  pub fn hitable_ptr(boundary: &HitablePtr, density: f64, phase_texture: TexturePtr) -> Arc<ConstantMedium> {
    Arc::new(ConstantMedium::new(boundary, density, phase_texture))
  }
}

impl Hitable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
      if let Some(mut hit1) = self.boundary.hit(ray, -std::f64::MAX, std::f64::MAX) {
        if let Some(mut hit2) = self.boundary.hit(ray, hit1.t + 0.0001, std::f64::MAX) {
          hit1.t = hit1.t.max(t_min);
          hit2.t = hit2.t.min(t_max);
          if hit1.t < 0.0 {
            return None;
          }
          let dist_inside_boundary = (hit2.t - hit1.t) * ray.direction.length();
          let hit_distance = -(1.0 / self.density) * rand_f64().log(std::f64::consts::E);
          if hit_distance < dist_inside_boundary {
            let t = hit1.t + hit_distance / ray.direction.length();
            let pt = ray.point_at_parameter(t);
            return Some(HitRecord::new(t, pt, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, self.material.clone()));

          }
        }
      }
      None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Aabb {
      self.boundary.bounding_box(time0, time1)
    }
}