use std::rc::Rc;

use vec3::Vec3;

pub trait Texture {
  // result: attenuation, scatter
  fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
  color: Vec3,
}

impl ConstantTexture {
  pub fn new(color: Vec3) -> ConstantTexture {
    ConstantTexture {
      color
    }
  }

  pub fn rc(color: Vec3) -> Rc<ConstantTexture> {
    Rc::new(ConstantTexture::new(color))
  }
}

impl Texture for ConstantTexture {
  fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
    self.color
  }
}

pub struct CheckerTexture {
  odd: Rc<Texture>,
  even: Rc<Texture>
}

impl CheckerTexture {
  pub fn new(odd: Rc<Texture>, even: Rc<Texture>) -> CheckerTexture {
    CheckerTexture {
      even,
      odd
    }
  }

  pub fn rc(odd: Rc<Texture>, even: Rc<Texture>) -> Rc<CheckerTexture> {
    Rc::new(CheckerTexture::new(odd, even))
  }
}

impl Texture for CheckerTexture {
  fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
    let period_factor: f64 = 10.0;
    let px: f64 = p.x * period_factor;
    let py: f64 = p.y * period_factor;
    let pz: f64 = p.z * period_factor;
    let sines: f64 = px.sin() * py.sin() * pz.sin();
    if sines < 0.0 {
      self.odd.value(u, v, p)
    } else {
      self.even.value(u, v, p)
    }
  }
}