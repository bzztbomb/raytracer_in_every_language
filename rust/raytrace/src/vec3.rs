use std::ops;

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

impl ops::Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, i: usize) -> &f64 {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => { panic!("Vec3[] invalid index: {}", i); }
    }
  }
}

impl ops::IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, i: usize) -> &mut f64 {
    match i {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => { panic!("Vec3[] invalid index: {}", i); }
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

