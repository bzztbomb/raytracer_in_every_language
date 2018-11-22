use std::rc::Rc;
use std::usize;
use std::mem;
use std::fmt;

use vec3::Vec3;
use ray::Ray;
use material::Material;
use aabb::Aabb;
use rt_rand::*;

#[derive(Clone)]
pub struct HitRecord {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3,
  pub u: f64,
  pub v: f64,
  pub material: Rc<Material>
}

impl HitRecord {
  fn new(t: f64, p: Vec3, normal: Vec3, u: f64, v: f64, material: Rc<Material>) -> HitRecord {
    HitRecord {
      t: t,
      p: p,
      normal: normal,
      u,
      v,
      material
    }
  }
}

impl fmt::Debug for HitRecord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "HitRecord {} {:?} {:?}", self.t, self.p, self.normal)
  }
}

pub trait Hitable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
  fn bounding_box(&self, _time0: f64, _time1: f64) -> Aabb;
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

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Aabb {
    panic!("Implement me?");
  }
}

struct BvhNode {
  left: Option<Box<Hitable>>,
  right: Option<Box<Hitable>>,
  bbox: Aabb,
}

impl BvhNode {
  fn boxed() -> Box<BvhNode> {
    Box::new(BvhNode {
      left: None,
      right: None,
      bbox: Aabb::new(Vec3::zero(), Vec3::zero()),
    })
  }
}

impl Hitable for BvhNode {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    if !self.bbox.hit(&ray, t_min, t_max) {
      return None
    }
    if let Some(ref left_hitable) = self.left {
      if let Some(ref right_hitable) = self.right {
        let hits = (
          left_hitable.hit(&ray, t_min, t_max),
          right_hitable.hit(&ray, t_min, t_max)
        );
        match hits {
          (None, None) => return None,
          (Some(left), None) => return Some(left),
          (None, Some(right)) => return Some(right),
          (Some(left), Some(right)) => if left.t < right.t {
            return Some(left);
          } else {
            return Some(right);
          }
        }
      } else {
        return left_hitable.hit(&ray, t_min, t_max);
      }
    } else {
      panic!("Empty node!!");
    }
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Aabb {
    self.bbox.clone()
  }
}
pub struct Bvh {
  root: Option<Box<Hitable>>,
}

impl Bvh {
  pub fn new(mut hitables: Vec<Box<Hitable>>, time0: f64, time1: f64) -> Bvh {
    let mut indices: Vec<usize> = Vec::with_capacity(hitables.len());
    for i in 0..hitables.len() {
      indices.push(i);
    }
    Bvh {
      root: Bvh::build(&mut hitables, &mut indices, time0, time1),
    }
  }

  fn build(hitables: &mut Vec<Box<Hitable>>, hitable_indices: &mut Vec<usize>, time0: f64, time1: f64) -> Option<Box<Hitable>> {
    // Alloc our bvh node
    let mut new_node = BvhNode::boxed();
    // We've hit the leaves
    let indices_len = hitable_indices.len();
    match indices_len {
      0 => panic!("Invalid!"),
      1 => {
        new_node.bbox = hitables[hitable_indices[0]].bounding_box(time0, time1);
        new_node.left = Some(mem::replace(&mut hitables[hitable_indices[0]], BvhNode::boxed()));
      },
      2 => {
        new_node.bbox = Aabb::surrounding_box(
          &hitables[hitable_indices[0]].bounding_box(time0, time1),
          &hitables[hitable_indices[1]].bounding_box(time0, time1)
        );
        new_node.left = Some(mem::replace(&mut hitables[hitable_indices[0]], BvhNode::boxed()));
        new_node.right = Some(mem::replace(&mut hitables[hitable_indices[1]], BvhNode::boxed()));
      },
      _ => {
        // Sort and divide the list
        let axis: usize = rand_usize() % 3;
        hitable_indices.sort_unstable_by(|a, b| {
          let a_aabb = hitables[*a].bounding_box(time0, time1);
          let b_aabb = hitables[*b].bounding_box(time0, time1);
          if let Some(res) = a_aabb.min[axis].partial_cmp(&b_aabb.min[axis]) {
            res
          } else {
            panic!("No NANS");
          }
        });
        let mut right_indices = hitable_indices.split_off(indices_len / 2);
        new_node.left = Bvh::build(hitables, hitable_indices, time0, time1);
        new_node.right = Bvh::build(hitables, &mut right_indices, time0, time1);
        let bbox;
        if let Some(ref l) = new_node.left {
          if let Some(ref r) = new_node.right {
            bbox = Aabb::surrounding_box(
              &l.bounding_box(time0, time1),
              &r.bounding_box(time0, time1));
          } else {
            panic!("I should be defined");
          }
        } else {
          panic!("I should be defined!");
        }
        new_node.bbox = bbox;
      }
    }
    Some(new_node)
  }
}

impl Hitable for Bvh {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    if let Some(ref root) = self.root {
      root.hit(ray, t_min, t_max)
    } else {
      None
    }
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Aabb {
    if let Some(ref root) = self.root {
      root.bounding_box(time0, time1)
    } else {
      Aabb {
        min: Vec3::zero(),
        max: Vec3::zero()
      }
    }
  }

}

pub struct Sphere {
  radius: f64,
  material: Rc<Material>,
  center: Box<Fn(f64) -> Vec3>,
}

 impl Sphere {

   pub fn new(c: Vec3, radius: f64, material: Rc<Material>) -> Sphere {
    Sphere {
      center: Box::new(move |_| c),
      radius,
      material
    }
  }

  pub fn boxed(center: Vec3, radius: f64, material: Rc<Material>) -> Box<Sphere> {
    Box::new(Sphere::new(center, radius, material))
  }

  pub fn new_moving(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, material: Rc<Material>) -> Sphere {
    Sphere {
      center: Box::new(move |t| center0 + ((t - time0) / (time1 - time0)) * (center1 - center0) ),
      radius,
      material
    }
  }

  pub fn boxed_moving(c0: Vec3, c1: Vec3, time0: f64, time1: f64, radius: f64, material: Rc<Material>) -> Box<Sphere> {
    Box::new(Sphere::new_moving(c0, c1, time0, time1, radius, material))
  }
}

impl Hitable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let center = (self.center)(ray.time);
    let oc = ray.origin - center;
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
        let normal = (pt - center) / self.radius;
        let phi = normal.z.atan2(normal.x);
        let theta = normal.y.asin();
        let u = 1.0-(phi+std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (theta + std::f64::consts::PI * 0.5) / std::f64::consts::PI;
        Some(HitRecord::new(t, pt, normal, u, v, self.material.clone()))
      } else {
        None
      }
    } else {
      None
    }
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Aabb {
    let sz = Vec3::one() * self.radius;
    let center0 = (self.center)(time0);
    let center1 = (self.center)(time1);
    Aabb::surrounding_box(
      &Aabb::new(center0 - sz, center0 + sz),
      &Aabb::new(center1 - sz, center1 + sz)
    )
  }
}