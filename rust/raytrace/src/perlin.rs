use vec3::Vec3;
use rt_rand::*;

pub struct Perlin {
  ran_vec: Vec<Vec3>,
  perm_x: Vec<usize>,
  perm_y: Vec<usize>,
  perm_z: Vec<usize>
}

impl Perlin {
  pub fn new() -> Perlin {
    Perlin {
      ran_vec: generate(),
      perm_x: generate_perm(),
      perm_y: generate_perm(),
      perm_z: generate_perm(),
    }
  }

  pub fn noise(&self, pt: &Vec3) -> f64 {
    let u = pt.x - pt.x.floor();
    let v = pt.y - pt.y.floor();
    let w = pt.z - pt.z.floor();
    let i = pt.x.floor() as usize;
    let j = pt.y.floor() as usize;
    let k = pt.z.floor() as usize;
    let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(), Vec3::zero()], [Vec3::zero(), Vec3::zero()]], [[Vec3::zero(), Vec3::zero()], [Vec3::zero(), Vec3::zero()]]];
    let mut cf: [[[f64; 2]; 2]; 2] = [[[0.0, 0.0], [0.0, 0.0]], [[0.0, 0.0], [0.0, 0.0]]];
    for di in 0_usize..2_usize {
      for dj in 0_usize..2_usize {
        for dk in 0_usize..2_usize {
          let idx = self.perm_x[(i.wrapping_add(di))&255] ^ self.perm_y[(j.wrapping_add(dj))&255] ^ self.perm_z[(k.wrapping_add(dk))&255];
          c[di][dj][dk] = self.ran_vec[idx];
        }
      }
    }
    perlin_interp(&c, u, v, w)
  }

  pub fn turb(&self, pt: &Vec3, depth: usize) -> f64 {
    let mut accum = 0.0;
    let mut temp_p = pt.clone();
    let mut weight = 1.0;
    for _i in 0..depth {
      accum += weight * self.noise(&temp_p);
      weight *= 0.5;
      temp_p = temp_p * 2.0;
    }
    accum.abs()
  }
}

fn triliner_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
  let mut accum = 0.0;
  for ii in 0_usize..2_usize {
    for ij in 0_usize..2_usize {
      for ik in 0_usize..2_usize {
        let i = ii as f64;
        let j = ij as f64;
        let k = ik as f64;
        let kernel = (i*u + (1.0-i)*(1.0-u)) * (j*v+ (1.0-j)*(1.0-v)) * (k*w + (1.0-k)*(1.0-w));
        accum += c[ii][ij][ik] * kernel
      }
    }
  }
  accum
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
  let uu = hermite(u);
  let vv = hermite(v);
  let ww = hermite(w);
  let mut accum = 0.0;
  for ii in 0_usize..2_usize {
    for ij in 0_usize..2_usize {
      for ik in 0_usize..2_usize {
        let i = ii as f64;
        let j = ij as f64;
        let k = ik as f64;
        let weight_v = Vec3::new(u-i, v-j, w-k);

        let kernel = (i*uu + (1.0-i)*(1.0-uu)) * (j*vv+ (1.0-j)*(1.0-vv)) * (k*ww + (1.0-k)*(1.0-ww));
        accum += Vec3::dot(&c[ii][ij][ik], &weight_v) * kernel;
      }
    }
  }
  accum
}

fn generate() -> Vec<Vec3> {
  let mut ret: Vec<Vec3> = Vec::with_capacity(256);
  for i in 0..256 {
    ret.push(Vec3::new(-1.0 + 2.0 * rand_f64(), -1.0 + 2.0 * rand_f64(), -1.0 + 2.0 * rand_f64()).normalized());
    // ret.push(Vec3::new(0.0, 0.25, 0.0));
  }
  ret
}

fn generate_float() -> Vec<f64> {
  let mut ret: Vec<f64> = Vec::with_capacity(256);
  for i in 0..256 {
    ret.push(rand_f64());
  }
  ret
}

fn generate_perm() -> Vec<usize> {
  let mut ret: Vec<usize> = Vec::with_capacity(256);
  for i in 0..256 {
    ret.push(i)
  }
  for i in 0..256 {
    let target = rand_usize() & 255;
    ret.swap(i, target);
  }
  ret
}

fn hermite(x: f64) -> f64 {
  x*x*(3.0-2.0*x)
}
