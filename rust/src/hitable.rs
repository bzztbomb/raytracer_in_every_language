use std::rc::Rc;

use vec3::Vec3;
use ray::Ray;
use material::Material;

#[derive(Clone)]
pub struct HitRecord {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: Rc<Material>
}

impl HitRecord {
  fn new(t: f64, p: Vec3, normal: Vec3, material: Rc<Material>) -> HitRecord {
    HitRecord {
      t: t,
      p: p,
      normal: normal,
      material
    }
  }
}

pub trait Hitable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList {
  list: Vec<Box<Hitable>>,
}

impl HitableList {
  pub fn new() -> HitableList {
    HitableList {
      list: vec![],
    }
  }

  pub fn add_hitable(&mut self, hitable: Box<Hitable>) {
    self.list.push(hitable);
  }
}

impl Hitable for HitableList {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut hit: Option<HitRecord> = None;
    let mut closest = t_max;
    for hitable in self.list.iter() {
      let result = hitable.hit(ray, t_min, closest);
      if let Some(hit_record) = result {
        hit = Some(hit_record.clone());
        closest = hit_record.t;
      }
    }
    hit
  }
}

pub struct Sphere {
  center: Vec3,
  radius: f64,
  material: Rc<Material>
}

 impl Sphere {

   pub fn new(center: Vec3, radius: f64, material: Rc<Material>) -> Sphere {
    Sphere {
      center,
      radius,
      material
    }
  }

  pub fn boxed(center: Vec3, radius: f64, material: Rc<Material>) -> Box<Sphere> {
    Box::new(Sphere {
      center,
      radius,
      material
    })
  }
}

impl Hitable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = ray.origin - self.center;
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = Vec3::dot(&oc, &ray.direction);
    let c = Vec3::dot(&oc, &oc) - self.radius*self.radius;
    let discriminent = b*b - a*c;
    if discriminent > 0.0 {
      let sq = discriminent.sqrt();
      let root = (-b - sq) / a;
      let t = if root > t_min && root < t_max {
        root
      } else {
        (-b + sq) / a
      };
      if t > t_min && t < t_max {
        let pt = ray.point_at_parameter(t);
        let normal = (pt - self.center) / self.radius;
        // let phi = normal.z.atan2(normal.x);
        // let theta = normal.y.asin();
        // let u = 1.0-(phi+std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        // let v = (theta + std::f64::consts::PI * 0.5) / std::f64::consts::PI;
        Some(HitRecord::new(t, pt, normal, self.material.clone()))
      } else {
        None
      }
    } else {
      None
    }
  }
}
