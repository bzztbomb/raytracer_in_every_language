use std::rc::Rc;

use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;
use rt_rand::*;
use texture::Texture;

pub struct ScatterInfo {
  pub attenuation: Vec3,
  pub scattered: Ray,
}

pub trait Material {
  // result: attenuation, scatter
  fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo>;
}

pub struct Lambertian {
  texture: Rc<Texture>
}

impl Lambertian {
  pub fn new(texture: Rc<Texture>) -> Lambertian {
    Lambertian {
      texture
    }
  }

  pub fn rc(texture: Rc<Texture>) -> Rc<Lambertian> {
    Rc::new(Lambertian::new(texture))
  }
}

impl Material for Lambertian {
  fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo> {
    let target = hit.p + hit.normal + random_in_unit_sphere();
    let scattered = Ray::new(hit.p, target - hit.p, ray.time);
    Some(ScatterInfo {
      attenuation: self.texture.value(hit.u, hit.v, &hit.p),
      scattered
    })
  }
}

pub struct Metal {
  texture: Rc<Texture>,
  fuzz: f64,
}

impl Metal {
  pub fn new(texture: Rc<Texture>, fuzz: f64) -> Metal {
    Metal {
      texture,
      fuzz
    }
  }

  pub fn rc(texture: Rc<Texture>, fuzz: f64) -> Rc<Metal> {
    Rc::new(Metal::new(texture, fuzz))
  }
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterInfo> {
    let reflected = Vec3::reflect(&ray.direction.normalized(), &hit.normal);
    let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere(), ray.time);
    if Vec3::dot(&scattered.direction, &hit.normal) > 0.0 {
      Some(ScatterInfo {
        attenuation: self.texture.value(hit.u, hit.v, &hit.p),
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

  pub fn rc(ref_index: f64) -> Rc<Dielectric> {
    Rc::new(Dielectric::new(ref_index))
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
    if rand_f64() < reflect_prob {
      Some(ScatterInfo {
        attenuation: Vec3::one(),
        scattered: Ray::new(hit.p, reflected, ray.time),
      })
    } else {
      Some(ScatterInfo {
        attenuation: Vec3::one(),
        scattered: Ray::new(hit.p, refracted, ray.time)
      })
    }
  }
}
