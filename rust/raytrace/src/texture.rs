extern crate image;

use std::rc::Rc;
use std::path::Path;
use self::image::{DynamicImage, GenericImageView, open, Pixel};

use vec3::Vec3;
use perlin::Perlin;

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
  fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
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

pub struct NoiseTexture {
  scale: f64,
  perlin: Perlin
}

impl NoiseTexture {
  pub fn new(scale: f64) -> NoiseTexture {
    NoiseTexture {
      scale,
      perlin: Perlin::new()
    }
  }

  pub fn rc(scale: f64) -> Rc<NoiseTexture> {
    Rc::new(NoiseTexture::new(scale))
  }
}

impl Texture for NoiseTexture {
  fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
    // let offset = (1.0 / 0.8) * 0.5;
    // Straight noise, range is -0.8..0.8
    // Vec3::one() * ((self.perlin.noise(&(*p * self.scale)) * offset) + offset)
    // Striaght turb
    // Vec3::one() * self.perlin.turb(&(self.scale * *p), 7)
    // let noise = 0.5 * (1.0 + (self.scale * p.x + 5.0 * self.perlin.turb(&(self.scale * *p), 7)).sin());
    let noise = 0.5 * (1.0 + (self.scale * p.z + 5.0 * self.perlin.turb(&(self.scale * *p), 7)).sin());
    noise * Vec3::one()
  }
}

pub struct ImageTexture {
  image: DynamicImage
}

impl ImageTexture {
  pub fn new(filename: &Path) -> ImageTexture {
    ImageTexture {
      image: open(filename).unwrap()
    }
  }

  pub fn rc(filename: &Path) -> Rc<ImageTexture> {
    Rc::new(ImageTexture::new(filename))
  }
}

impl Texture for ImageTexture {
  fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
    let (width, height) = self.image.dimensions();
    let i = (u * width as f64).min(width as f64);
    let j = ((1.0 - v) * height as f64 - 0.001).min(height as f64);
    let pixel = self.image.get_pixel(i as u32, j as u32).to_rgb();
    let ret = Vec3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);
    ret / 255.0
  }
}