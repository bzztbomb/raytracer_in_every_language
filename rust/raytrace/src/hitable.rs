use std::sync::Arc;
use std::usize;
use std::fmt;

use vec3::Vec3;
use ray::Ray;
use material::MaterialPtr;
use aabb::Aabb;
use rt_rand::*;

#[derive(Clone)]
pub struct HitRecord {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3,
  pub u: f64,
  pub v: f64,
  pub material: MaterialPtr
}

impl HitRecord {
  pub fn new(t: f64, p: Vec3, normal: Vec3, u: f64, v: f64, material: MaterialPtr) -> HitRecord {
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

pub type HitablePtr = Arc<Hitable + Sync + Send>;

pub struct HitableList {
  pub list: Vec<HitablePtr>,
}

impl HitableList {
  pub fn new() -> HitableList {
    HitableList {
      list: vec![],
    }
  }

  pub fn add_hitable(&mut self, hitable: HitablePtr) {
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
  left: Option<HitablePtr>,
  right: Option<HitablePtr>,
  bbox: Aabb,
}

impl BvhNode {
  fn hitable_ptr() -> Arc<BvhNode> {
    Arc::new(BvhNode {
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
  root: Option<HitablePtr>,
}

impl Bvh {
  pub fn new(mut hitables: Vec<HitablePtr>, time0: f64, time1: f64) -> Bvh {
    let mut indices: Vec<usize> = Vec::with_capacity(hitables.len());
    for i in 0..hitables.len() {
      indices.push(i);
    }
    Bvh {
      root: Bvh::build(&mut hitables, &mut indices, time0, time1),
    }
  }

  fn build(hitables: &mut Vec<HitablePtr>, hitable_indices: &mut Vec<usize>, time0: f64, time1: f64) -> Option<HitablePtr> {
    // Alloc our bvh node
    let mut new_node_rc = BvhNode::hitable_ptr();
    {
      let new_node = Arc::get_mut(&mut new_node_rc).unwrap();
      // We've hit the leaves
      let indices_len = hitable_indices.len();
      match indices_len {
        0 => panic!("Invalid!"),
        1 => {
          new_node.bbox = hitables[hitable_indices[0]].bounding_box(time0, time1);
          new_node.left = Some(Arc::clone(&hitables[hitable_indices[0]]));
        },
        2 => {
          new_node.bbox = Aabb::surrounding_box(
            &hitables[hitable_indices[0]].bounding_box(time0, time1),
            &hitables[hitable_indices[1]].bounding_box(time0, time1)
          );
          new_node.left = Some(Arc::clone(&hitables[hitable_indices[0]]));
          new_node.right = Some(Arc::clone(&hitables[hitable_indices[1]]));
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
    }
    Some(new_node_rc)
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

pub struct FlipNormals {
  hitable: HitablePtr,
}

impl FlipNormals {
  pub fn new(hitable: HitablePtr) -> FlipNormals {
    FlipNormals {
      hitable: Arc::clone(&hitable)
    }
  }

  pub fn hitable_ptr(hitable: HitablePtr) -> Arc<FlipNormals> {
    Arc::new(FlipNormals::new(hitable))
  }
}

impl Hitable for FlipNormals {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    if let Some(mut ret) = self.hitable.hit(ray, t_min, t_max) {
      ret.normal = ret.normal * -1.0;
      Some(ret)
    } else {
      None
    }
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Aabb {
    self.hitable.bounding_box(time0, time1)
  }
}

pub struct Translate {
  hitable: HitablePtr,
  offset: Vec3,
}

impl Translate {
  pub fn new(hitable: HitablePtr, offset: Vec3) -> Translate {
    Translate {
      hitable,
      offset
    }
  }

  pub fn hitable_ptr(hitable: HitablePtr, offset: Vec3) -> HitablePtr {
    Arc::new(Translate::new(hitable, offset))
  }
}

impl Hitable for Translate {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let ray_moved = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
    if let Some(mut ret) = self.hitable.hit(&ray_moved, t_min, t_max) {
      ret.p = ret.p + self.offset;
      Some(ret)
    } else {
      None
    }
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Aabb {
    let mut ret = self.hitable.bounding_box(time0, time1);
    ret.min = ret.min + self.offset;
    ret.max = ret.max + self.offset;
    ret
  }
}

pub struct RotateY {
  hitable: HitablePtr,
  sin_theta: f64,
  cos_theta: f64,
  aabb: Aabb
}

impl RotateY {
  pub fn new(hitable: HitablePtr, angle_degrees: f64) -> RotateY {
    let radians = angle_degrees * (std::f64::consts::PI / 180.0);
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();
    let aabb = hitable.bounding_box(0.0, std::f64::MAX);
    let m = std::f64::MAX;
    let mut minb = Vec3::new(m, m, m);
    let mut maxb = minb * -1.0;
    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          let x = i as f64 * aabb.max.x + (1.0 - i as f64) * aabb.min.x;
          let y = j as f64 * aabb.max.y + (1.0 - j as f64) * aabb.min.y;
          let z = k as f64 * aabb.max.z + (1.0 - k as f64) * aabb.min.z;
          let newx = cos_theta * x + sin_theta * z;
          let newz = -sin_theta * x + cos_theta * z;
          let tester = Vec3::new(newx, y, newz);
          for c in 0..3 {
            minb[c] = minb[c].min(tester[c]);
            maxb[c] = maxb[c].max(tester[c]);
          }
        }
      }
    }
    RotateY {
      hitable,
      sin_theta,
      cos_theta,
      aabb: Aabb::new(minb, maxb)
    }
  }

  pub fn hitable_ptr(hitable: HitablePtr, angle_degrees: f64) -> Arc<RotateY> {
    Arc::new(RotateY::new(hitable, angle_degrees))
  }

  fn rotate_vec3(&self, v: Vec3) -> Vec3 {
    Vec3::new(self.cos_theta * v.x - self.sin_theta * v.z, v.y, self.sin_theta * v.x + self.cos_theta * v.z)
  }

  fn inverse_rotate_vec3(&self, v: Vec3) -> Vec3 {
    Vec3::new(self.cos_theta * v.x + self.sin_theta * v.z, v.y, -self.sin_theta * v.x + self.cos_theta * v.z)
  }
}

impl Hitable for RotateY {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let origin = self.rotate_vec3(ray.origin);
    let direction = self.rotate_vec3(ray.direction);
    let rotated_ray = Ray::new(origin, direction, ray.time);
    if let Some(mut ret) = self.hitable.hit(&rotated_ray, t_min, t_max) {
      ret.p = self.inverse_rotate_vec3(ret.p);
      ret.normal = self.inverse_rotate_vec3(ret.normal);
      Some(ret)
    } else {
      None
    }
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Aabb {
    self.aabb.clone()
  }
}

pub struct Sphere {
  radius: f64,
  material: MaterialPtr,
  center: Box<Fn(f64) -> Vec3 + Sync + Send>,
}

 impl Sphere {

   pub fn new(c: Vec3, radius: f64, material: MaterialPtr) -> Sphere {
    Sphere {
      center: Box::new(move |_| c),
      radius,
      material
    }
  }

  pub fn hitable_ptr(center: Vec3, radius: f64, material: MaterialPtr) -> Arc<Sphere> {
    Arc::new(Sphere::new(center, radius, material))
  }

  pub fn new_moving(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, material: MaterialPtr) -> Sphere {
    Sphere {
      center: Box::new(move |t| center0 + ((t - time0) / (time1 - time0)) * (center1 - center0) ),
      radius,
      material
    }
  }

  pub fn hitable_ptr_moving(c0: Vec3, c1: Vec3, time0: f64, time1: f64, radius: f64, material: MaterialPtr) -> Arc<Sphere> {
    Arc::new(Sphere::new_moving(c0, c1, time0, time1, radius, material))
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

struct AARect {
  a_index: usize,
  b_index: usize,
  c_index: usize,
  a0: f64,
  b0: f64,
  a1: f64,
  b1: f64,
  c: f64,
  material: MaterialPtr,
  a_range: f64,
  b_range: f64,
}

impl AARect {
  pub fn new(a_index: usize, b_index: usize, c_index: usize, a0: f64, b0: f64, a1: f64, b1: f64, c: f64, material: MaterialPtr) -> AARect {
    AARect {
      a_index,
      b_index,
      c_index,
      a0,
      b0,
      a1,
      b1,
      c,
      material,
      a_range: a1 - a0,
      b_range: b1 - b0
    }
  }

  pub fn hitable_ptr(a_index: usize, b_index: usize, c_index: usize, a0: f64, b0: f64, a1: f64, b1: f64, c: f64, material: MaterialPtr) -> Arc<AARect> {
    Arc::new(AARect::new(a_index, b_index, c_index, a0, b0, a1, b1, c, material))
  }
}

impl Hitable for AARect {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    if ray.direction[self.c_index] == 0.0 {
      // println!("Case 1");
      return None;
    }
    let t = (self.c - ray.origin[self.c_index]) / ray.direction[self.c_index];
    if t < t_min || t > t_max {
      // println!("Case 2, t: {} t_min: {} t_max: {}", t, t_min, t_max);
      return None
    }
    let a = ray.origin[self.a_index] + t * ray.direction[self.a_index];
    if a < self.a0 || a > self.a1 {
      // println!("Case 3");
      return None
    }
    let b = ray.origin[self.b_index] + t * ray.direction[self.b_index];
    if b < self.b0 || b > self.b1 {
      // println!("Case 4");
      return None
    }
    let u = (a - self.a0) / self.a_range;
    let v = (b - self.b0) / self.b_range;

    let pt = ray.point_at_parameter(t);
    let mut normal = Vec3::zero();
    normal[self.c_index] = 1.0;
    Some(HitRecord::new(t, pt, normal, u, v, self.material.clone()))
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Aabb {
    let eplison = 0.0001;
    let mut b_min = Vec3::zero();
    b_min[self.a_index] = self.a0;
    b_min[self.b_index] = self.b0;
    b_min[self.c_index] = self.c - eplison;
    let mut b_max = Vec3::zero();
    b_max[self.a_index] = self.a1;
    b_max[self.b_index] = self.b1;
    b_max[self.c_index] = self.c + eplison;
    Aabb::new(b_min, b_max)
  }
}

pub struct Rect {
  //
}

impl Rect {
  pub fn xyrect(x0: f64, y0: f64, x1: f64, y1: f64, k: f64, material: MaterialPtr) -> HitablePtr {
    AARect::hitable_ptr(0, 1, 2, x0, y0, x1, y1, k, material)
  }

  pub fn xzrect(x0: f64, z0: f64, x1: f64, z1: f64, k: f64, material: MaterialPtr) -> HitablePtr {
    AARect::hitable_ptr(0, 2, 1, x0, z0, x1, z1, k, material)
  }

  pub fn yzrect(y0: f64, z0: f64, y1: f64, z1: f64, k: f64, material: MaterialPtr) -> HitablePtr {
    AARect::hitable_ptr(1, 2, 0, y0, z0, y1, z1, k, material)
  }
}

pub struct AabbBox {
  aabb: Aabb,
  faces: HitableList,
}

impl AabbBox {
  pub fn new(aabb: Aabb, material: MaterialPtr) -> AabbBox {
    let mut faces = HitableList::new();
    faces.add_hitable(Rect::xyrect(aabb.min.x, aabb.min.y, aabb.max.x, aabb.max.y, aabb.max.z, Arc::clone(&material)));
    faces.add_hitable(FlipNormals::hitable_ptr(Rect::xyrect(aabb.min.x, aabb.min.y, aabb.max.x, aabb.max.y, aabb.min.z, Arc::clone(&material))));
    faces.add_hitable(Rect::xzrect(aabb.min.x, aabb.min.z, aabb.max.x, aabb.max.z, aabb.max.y, Arc::clone(&material)));
    faces.add_hitable(FlipNormals::hitable_ptr(Rect::xzrect(aabb.min.x, aabb.min.z, aabb.max.x, aabb.max.z, aabb.min.y, Arc::clone(&material))));
    faces.add_hitable(Rect::yzrect(aabb.min.y, aabb.min.z, aabb.max.y, aabb.max.z, aabb.max.x, Arc::clone(&material)));
    faces.add_hitable(FlipNormals::hitable_ptr(Rect::yzrect(aabb.min.y, aabb.min.z, aabb.max.y, aabb.max.z, aabb.min.x, Arc::clone(&material))));
    AabbBox {
      aabb,
      faces
    }
  }

  pub fn hitable_ptr(aabb: Aabb, material: MaterialPtr) -> Arc<AabbBox> {
    Arc::new(AabbBox::new(aabb, material))
  }
}

impl Hitable for AabbBox {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    self.faces.hit(ray, t_min, t_max)
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Aabb {
    self.aabb.clone()
  }
}

#[cfg(test)]
mod tests {

  use vec3::Vec3;
  use material::Lambertian;
  use texture::ConstantTexture;
  use ray::Ray;
  use hitable::*;

  #[test]
  fn test_xzrect() {
    let mat: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(1.0, 1.0, 1.0)));
    let xz = Rect::xzrect(0.0, 0.0, 555.0, 555.0, 0.0, Arc::clone(&mat));
    let ray = Ray::new(Vec3::new(100.0, 4.0, 100.0), Vec3::new(0.0, -1.0, 0.0), 0.0);
    let hit = xz.hit(&ray, 0.0, std::f64::MAX);

    assert!(hit.is_some());
    if let Some(hit_record) = hit {
      assert_eq!(hit_record.t, 4.0);
      assert_eq!(hit_record.p, Vec3::new(100.0, 0.0, 100.0));
      assert_eq!(hit_record.normal, Vec3::new(0.0, 1.0, 0.0));
    }

    let xz2 = Rect::xzrect(213.0, 227.0, 343.0, 332.0, 554.0, Arc::clone(&mat));
    let ray2 = Ray::new(Vec3::new(278.0, 278.0, -800.0), Vec3::new(0.0, 2.5477916398634193, 10.0), 0.0);
    let hit2 = xz2.hit(&ray2, 0.0, std::f64::MAX);
    assert!(hit2.is_some());
    if let Some(hit2) = hit2 {
      assert_eq!(hit2.t, 108.32910968135356);
      assert_eq!(hit2.p, Vec3::new(278.0, 554.0, 283.29109681353566));
      assert_eq!(hit2.normal, Vec3::new(0.0, 1.0, 0.0));
    }
  }
}