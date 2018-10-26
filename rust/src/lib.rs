extern crate rand;

use std::ops;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    pub fn zero() -> Vec3 {
      Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      }
    }

    pub fn one() -> Vec3 {
      Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
      }
    }

    pub fn length(&self) -> f64 {
      (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
      let l = self.length();
      if l > 0.0 {
        let factor = 1.0 / l;
        Vec3 {
          x: self.x * factor,
          y: self.y * factor,
          z: self.z * factor,
        }
      } else {
        Vec3::zero()
      }
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
      a.x*b.x + a.y*b.y + a.z*b.z
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
      Vec3::new(a.y*b.z - a.z*b.y, -(a.x*b.z - a.z*b.x), a.x*b.y - a.y*b.x)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
      *v - *n * 2.0 * Vec3::dot(v, n)
    }

    pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
      let uv = v.normalized();
      let dt = Vec3::dot(&uv, n);
      let discr = 1.0 - ni_over_nt*ni_over_nt * (1.0 - dt*dt);
      if discr > 0.0 {
        Some((uv -*n * dt) * ni_over_nt - *n * discr.sqrt())
      } else {
        None
      }
    }
}


impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
    }
  }
}

impl ops::Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Vec3 {
    Vec3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl ops::Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self * rhs.x,
      y: self * rhs.y,
      z: self * rhs.z,
    }
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f64) -> Vec3 {
    Vec3 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

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

fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = 2.0 * Vec3::new(rand::random::<f64>(), rand::random::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    if Vec3::dot(&p, &p) < 1.0  {
      break p;
    }
  }
}

pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  lens_radius: f64
}

impl Camera {
  pub fn new(look_from: &Vec3, look_at: &Vec3, v_up: &Vec3, vfov: f64, aspect: f64, aperature: f64, focus_dist: f64) -> Camera {
    let theta = vfov * std::f64::consts::PI / 180.0;
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
      w,
      lens_radius: aperature / 2.0
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.x + self.v * rd.y;
    Ray::new(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
  }
}

pub struct ScatterInfo {
  pub attenuation: Vec3,
  pub scattered: Ray,
}

pub trait Material {
  // result: attenuation, scatter
  fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo>;
}

pub struct Lambertian {
  albedo: Vec3
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Lambertian {
    Lambertian {
      albedo
    }
  }
}

fn random_in_unit_sphere() -> Vec3 {
    // Generate a vector with spherical coords y0
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) - Vec3::one();
        if Vec3::dot(&p, &p) < 1.0 {
            break p;
        }
    }
}

impl Material for Lambertian {
  fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo> {
    let target = hit.p + hit.normal + random_in_unit_sphere();
    let scattered = Ray::new(hit.p, target - hit.p);
    Some(ScatterInfo {
      attenuation: self.albedo,
      scattered
    })
  }
}

pub struct Metal {
  albedo: Vec3,
  fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
    Metal {
      albedo,
      fuzz
    }
  }
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo> {
    let reflected = Vec3::reflect(&ray.direction.normalized(), &hit.normal);
    let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
    if Vec3::dot(&scattered.direction, &hit.normal) > 0.0 {
      Some(ScatterInfo {
        attenuation: self.albedo,
        scattered
      })
    } else {
      None
    }
  }
}

pub struct Dielectric {
  ref_index: f64,
}

impl Dielectric {
  pub fn new(ref_index: f64) -> Dielectric {
    return Dielectric {
      ref_index
    }
  }
}

fn schlick(cosine: f64, ref_index: f64) -> f64 {
  let r0 = (1.0 - ref_index) / (1.0 + ref_index);
  let r2 = r0*r0;
  r2 + (1.0 - r2) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
  fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo> {
    let reflected = Vec3::reflect(&ray.direction, &hit.normal);
    let ni_over_nt: f64;
    let outward_normal: Vec3;
    let cosine: f64;
    if Vec3::dot(&ray.direction.normalized(), &hit.normal) > 0.0 {
      outward_normal = hit.normal * -1.0;  // todo: unary
      ni_over_nt = self.ref_index;
      let c = Vec3::dot(&ray.direction, &hit.normal) / ray.direction.length();
      let g = 1.0 - self.ref_index*self.ref_index*(1.0 - c*c);
      cosine = if g > 0.0 {
        g.sqrt()
      } else {
        0.0
      };
    } else {
      outward_normal = hit.normal;
      ni_over_nt = 1.0 / self.ref_index;
      cosine = Vec3::dot(&(ray.direction.normalized() * -1.0), &hit.normal.normalized())
    }
    let refracted0 = Vec3::refract(&ray.direction, &outward_normal, ni_over_nt);
    let mut refracted = Vec3::zero();
    let reflect_prob;
    if let Some(r) = refracted0 {
      reflect_prob = schlick(cosine, self.ref_index);
      refracted = r;
    } else {
      reflect_prob = 1.0;
    }
    if rand::random::<f64>() < reflect_prob {
      Some(ScatterInfo {
        attenuation: Vec3::one(),
        scattered: Ray::new(hit.p, reflected),
      })
    } else {
      Some(ScatterInfo {
        attenuation: Vec3::one(),
        scattered: Ray::new(hit.p, refracted)
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pt_at_param() {
    let r = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0));
    let p = r.point_at_parameter(0.5);
    let expected = Vec3::new(1.0, 1.5, 1.0);
    assert_eq!(expected, p);
  }

  #[test]
  fn test_ops() {
    assert_eq!(Vec3::new(0.0, 1.0, 0.0).length(), 1.0);
    assert_eq!(Vec3::new(10.0, 0.0, 0.0).normalized(), Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(Vec3::cross(&Vec3::new(0.0, 0.0, 1.0), &Vec3::new(0.0, 1.0, 0.0)), Vec3::new(-1.0, 0.0, 0.0));
  }
}

