extern crate rand;

use vec3::Vec3;
use std::cell::RefCell;
use rt_rand::rand::RngCore;

use rt_rand::rand::random;

pub fn rand_f64() -> f64 {
  return random::<f64>()
}

pub fn rand_usize() -> usize {
  return random::<usize>()
}

// thread_local! {
//   static PCG: RefCell<rand_pcg::Mcg128Xsl64> = RefCell::new(rand_pcg::Mcg128Xsl64::new(2084));
// }

// pub fn rand_f64() -> f64 {
//   PCG.with(|pcg| {
//     pcg.borrow_mut().next_u64() as f64 / (std::u64::MAX as f64)
//   })
// }

// pub fn rand_usize() -> usize {
//   PCG.with(|pcg| {
//     pcg.borrow_mut().next_u64() as usize
//   })
// }

pub fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = 2.0 * Vec3::new(rand_f64(), rand_f64(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    if Vec3::dot(&p, &p) < 1.0  {
      break p;
    }
  }
}

pub fn random_in_unit_sphere() -> Vec3 {
    // Generate a vector with spherical coords y0
    loop {
        let p = 2.0 * Vec3::new(rand_f64(), rand_f64(), rand_f64()) - Vec3::one();
        if Vec3::dot(&p, &p) < 1.0 {
            break p;
        }
    }
}
