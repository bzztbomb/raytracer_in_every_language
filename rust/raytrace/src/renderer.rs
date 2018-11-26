use camera::Camera;
use hitable::Hitable;
use scenes::*;
use vec3::Vec3;
use ray::Ray;
use rt_rand::*;

pub struct Renderer {
  scene: Box<Hitable>,
  camera: Camera,
  nx: u32,
  ny: u32,
  num_samples: u32,
  default_sky: bool
}

impl Renderer {
  // TODO: Pick scene externally.
  pub fn new(nx: u32, ny: u32, ns: u32) -> Renderer {
    let (scene, camera, default_sky) = scene_cornell(nx, ny);
    Renderer {
      scene, camera, nx, ny, num_samples: ns, default_sky
    }
  }

  pub fn pixel_color(&self, i: u32, j: u32) -> Vec3 {
    let mut c = Vec3::zero();
    for _ in 0..self.num_samples {
        let u = ((i as f64) + rand_f64()) / self.nx as f64;
        let v = ((j as f64) + rand_f64()) / self.ny as f64;
        let r = self.camera.get_ray(u, v);
        let p = self.color(&r, &self.scene, 0);
        c = c + p;
    }
    c = c / self.num_samples as f64;
    c.x = c.x.sqrt();
    c.y = c.y.sqrt();
    c.z = c.z.sqrt();
    let m = c.x.max(c.y.max(c.z));
    if m > 1.0 {
      c = c / m;
    }
    c * 255.0
  }

  fn color(&self, r: &Ray, scene: &Box<Hitable>, depth: u32) -> Vec3 {
    if let Some(scene_hit) = scene.hit(r, 0.001, std::f64::MAX) {
        if depth >= 50 {
            return Vec3::zero();
        }
        let emitted = scene_hit.material.emit(scene_hit.u, scene_hit.v, &scene_hit.p);
        if let Some(scatter) = scene_hit.material.scatter(&r, &scene_hit) {
            return emitted + (scatter.attenuation * self.color(&scatter.scattered, scene, depth+1));
        } else {
            return emitted;
        }
    }
    if !self.default_sky {
      Vec3::zero()
    } else {
      let unit_direction = r.direction.normalized();
      let t = 0.5 * (unit_direction.y + 1.0);
      (1.0 - t)*Vec3::one() + t*Vec3::new(0.5, 0.7, 1.0)
    }
  }
}