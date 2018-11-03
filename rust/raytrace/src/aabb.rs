use std::mem;
use vec3::Vec3;
use ray::Ray;

#[derive(Clone, Debug)]
pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3,
}

impl Aabb {
  pub fn new(min: Vec3, max: Vec3) -> Aabb {
    Aabb {
      min,
      max
    }
  }

  pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
    for a in 0..3 {
      if ray.direction[a] == 0.0 {
        println!("skipping {}", a);
        continue;
      }
      let inv_d = 1.0 / ray.direction[a];
      let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
      let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
      if inv_d < 0.0 {
        mem::swap(&mut t0, &mut t1);
      }
      let mn = if t0 > t_min { t0 } else { t_min };
      let mx = if t1 < t_max { t1 } else { t_max };
      if mx <= mn {
        return false;
      }
    }
    true
  }

  pub fn surrounding_box(a: &Aabb, b: &Aabb) -> Aabb {
    Aabb {
      min: Vec3::new(a.min.x.min(b.min.x), a.min.y.min(b.min.y), a.min.z.min(b.min.z)),
      max: Vec3::new(a.max.x.max(b.max.x), a.max.y.max(b.max.y), a.max.z.max(b.max.z)),
    }
  }
}