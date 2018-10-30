extern crate rust;
extern crate rand;

use rand::random;

use rust::vec3::Vec3;
use rust::ray::Ray;
use rust::hitable::*;
use rust::material::*;
use rust::camera::*;

fn color(r: &Ray, scene: &Box<Hitable>, depth: u32) -> Vec3 {
    if let Some(scene_hit) = scene.hit(r, 0.0, std::f64::MAX) {
        // println!("Scene hit: {:?}", scene_hit);
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
const NS: u32 = 200;

fn main() {
    let (scene, camera) = simple_scene();

    // let debug_r = camera.get_ray(131.0 / NX as f64, 98.0 / NY as f64);
    // println!("ray: {:?}\n", debug_r);
    // let p = color(&debug_r, &scene, 0);
    // println!("{:?}\n", p);

    println!("P3\n{} {}\n255", NX, NY);
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut c = Vec3::zero();
            for _ in 0..NS {
                let u = ((i as f64) + random::<f64>()) / NX as f64;
                let v = ((j as f64) + random::<f64>()) / NY as f64;
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

#[allow(dead_code)]
fn simple_scene() -> (Box<Hitable>, Camera) {
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, NX as f64 / NY as f64, aperture, dist_to_focus, 0.0, 0.0);

    let mut objs: Vec<Box<Hitable>> = vec![
        Sphere::boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::rc(Vec3::new(0.5, 0.5, 0.5))),
        Sphere::boxed(Vec3::new(0.0, -100.5, 0.0), 100.0, Lambertian::rc(Vec3::new(0.0, 1.0, 0.0))),
        Sphere::boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::rc(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        // Sphere::boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Metal::rc(Vec3::new(0.8, 0.8, 0.8), 1.0)),
        Sphere::boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::rc(1.5)),
        Sphere::boxed(Vec3::new(-1.0, 0.0, -1.0), -0.45, Dielectric::rc(1.5)),
    ];

    let result = Box::new(Bvh::new(objs, 0.0, 1.0));
    // let mut result = Box::new(HitableList::new());
    // while let Some(obj) = objs.pop() {
    //     result.add_hitable(obj);
    // }
    (result, camera)
}

#[allow(dead_code)]
fn scene_random() -> (Box<Hitable>, Camera) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.01;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, NX as f64 / NY as f64, aperture, dist_to_focus, 0.0, 1.0);

    let mut result = Box::new(HitableList::new());
    result.add_hitable(Sphere::boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::rc(Vec3::new(0.5, 0.5, 0.5))));
	result.add_hitable(Sphere::boxed(Vec3::new(0.0,1.0,0.0), 1.0, Dielectric::rc(1.5)));
	result.add_hitable(Sphere::boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::rc(Vec3::new(0.4, 0.2, 0.1))));
	result.add_hitable(Sphere::boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, Metal::rc(Vec3::new(0.7, 0.6, 0.5), 0.0)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let radius = 0.2;
            let center = Vec3::new(a as f64+0.9+random::<f64>(), radius, b as f64+0.9+random::<f64>());
            let offset = Vec3::new(4.0, radius, 0.0);
            if (center - offset).length() > 0.9 {
                if choose_mat < 0.8 {
                    let lam_rand = || { random::<f64>()*random::<f64>() };
                    let mat = Lambertian::rc(Vec3::new(lam_rand(), lam_rand(), lam_rand()));
                    let center1 = center + Vec3::new(0.0, random::<f64>() * 0.15, 0.0);
                    result.add_hitable(Sphere::boxed_moving(center, center1, 0.0, 1.0, radius, mat));
                } else if choose_mat < 0.95 {
                    let met_rand = || (1.0 + random::<f64>()) * 0.5;
                    let mat = Metal::rc(
                        Vec3::new(met_rand(), met_rand(), met_rand()), random::<f64>() * 0.5);
                    result.add_hitable(Sphere::boxed(center, radius, mat));
                } else {
                    let mat = Dielectric::rc(1.5);
                    result.add_hitable(Sphere::boxed(center, radius, mat));
                }
            }
        }
    }

    (result, camera)

}

