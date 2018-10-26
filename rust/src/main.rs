extern crate rust;
extern crate rand;

use rust::Vec3;
use rust::Ray;
use rust::HitableList;
use rust::Sphere;
use rust::Hitable;
use rust::Camera;
use rust::Material;
use rust::Lambertian;
use rust::Metal;
use rust::Dielectric;
use std::rc::Rc;

fn color(r: &Ray, scene: &Box<Hitable>, depth: u32) -> Vec3 {
    if let Some(scene_hit) = scene.hit(r, 0.0, 10000.0) {
        if depth >= 50 {
            return Vec3::zero();
        }
        if let Some(scatter) = scene_hit.material.scatter(&r, &scene_hit) {
            return scatter.attenuation * color(&scatter.scattered, scene, depth+1);
        } else {
            return Vec3::zero();
        }
    }
    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t)*Vec3::one() + t*Vec3::new(0.5, 0.7, 1.0)
}

const NX: u32 = 400;
const NY: u32 = 200;
const NS: u32 = 100;

fn main() {
    println!("P3\n{} {}\n255", NX, NY);
    let (scene, camera) = scene_random();
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut c = Vec3::zero();
            for _ in 0..NS {
                let u = ((i as f64) + rand::random::<f64>()) / NX as f64;
                let v = ((j as f64) + rand::random::<f64>()) / NY as f64;
                let r = camera.get_ray(u, v);
                let p = color(&r, &scene, 0);
                c = c + p;
            }
            c = c / NS as f64;
            c.x = c.x.sqrt();
            c.y = c.y.sqrt();
            c.z = c.z.sqrt();
            c = c * 255.0;
            println!("{} {} {}", c.x as i32, c.y as i32, c.z as i32);
        }
    }
}

fn simple_scene() -> (Box<Hitable>, Camera) {
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, NX as f64 / NY as f64, aperture, dist_to_focus);

    let mut result = Box::new(HitableList::new());
    result.add_hitable(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
    result.add_hitable(Box::new(Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.0, 1.0, 0.0))))));
    result.add_hitable(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)))));
    // result.add_hitable(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0)))));
    result.add_hitable(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)))));
    result.add_hitable(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(1.5)))));
    (result, camera)
}

fn scene_random() -> (Box<Hitable>, Camera) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, NX as f64 / NY as f64, aperture, dist_to_focus);

    let mut result = Box::new(HitableList::new());
    result.add_hitable(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
	result.add_hitable(Box::new(Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, Rc::new(Dielectric::new(1.5)))));
	result.add_hitable(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
	result.add_hitable(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let radius = 0.2;
            let center = Vec3::new(a as f64+0.9+rand::random::<f64>(), radius, b as f64+0.9+rand::random::<f64>());
            let offset = Vec3::new(4.0, radius, 0.0);
            if (center - offset).length() > 0.9 {
                let mat: Rc<Material>;
                if choose_mat < 0.8 {
                    let lam_rand = || { rand::random::<f64>()*rand::random::<f64>() };
                    mat = Rc::new(Lambertian::new(Vec3::new(lam_rand(), lam_rand(), lam_rand())));
                } else if choose_mat < 0.95 {
                    let met_rand = || (1.0 + rand::random::<f64>()) * 0.5;
                    mat = Rc::new(Metal::new(
                        Vec3::new(met_rand(), met_rand(), met_rand()), rand::random::<f64>() * 0.5));
                } else {
                    mat = Rc::new(Dielectric::new(1.5));
                }
                result.add_hitable(Box::new(Sphere::new(center, radius, mat)))
            }
        }
    }

    (result, camera)

}

