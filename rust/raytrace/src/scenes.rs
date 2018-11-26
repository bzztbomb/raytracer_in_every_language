use std::rc::Rc;
use std::path::Path;

use vec3::Vec3;
use hitable::*;
use material::*;
use camera::*;
use rt_rand::*;
use texture::*;

pub fn simple_scene(nx: u32, ny: u32) -> (Box<Hitable>, Camera, bool) {
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 0.0);

    let objs: Vec<Box<Hitable>> = vec![
        Sphere::boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::rc(ConstantTexture::rc(Vec3::new(0.5, 0.5, 0.5)))),
        Sphere::boxed(Vec3::new(0.0, -100.5, 0.0), 100.0, Lambertian::rc(ConstantTexture::rc(Vec3::new(0.0, 1.0, 0.0)))),
        Sphere::boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::rc(ConstantTexture::rc(Vec3::new(0.8, 0.6, 0.2)), 0.3)),
        // Sphere::boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Metal::rc(Vec3::new(0.8, 0.8, 0.8), 1.0)),
        Sphere::boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::rc(1.5)),
        Sphere::boxed(Vec3::new(-1.0, 0.0, -1.0), -0.45, Dielectric::rc(1.5)),
    ];

    let result = Box::new(Bvh::new(objs, 0.0, 1.0));
    // let mut result = Box::new(HitableList::new());
    // while let Some(obj) = objs.pop() {
    //     result.add_hitable(obj);
    // }
    (result, camera, true)
}

pub fn scene_random(nx: u32, ny: u32) -> (Box<Hitable>, Camera, bool) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.01;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let mut result = Box::new(HitableList::new());
    result.add_hitable(Sphere::boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0,
        Lambertian::rc(CheckerTexture::rc(
            ConstantTexture::rc(Vec3::new(0.2, 0.3, 0.1)),
            ConstantTexture::rc(Vec3::new(0.9, 0.9, 0.9))
        ))));
	result.add_hitable(Sphere::boxed(Vec3::new(0.0,1.0,0.0), 1.0, Dielectric::rc(1.5)));
	result.add_hitable(Sphere::boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::rc(ConstantTexture::rc(Vec3::new(0.4, 0.2, 0.1)))));
	result.add_hitable(Sphere::boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, Metal::rc(ConstantTexture::rc(Vec3::new(0.7, 0.6, 0.5)), 0.0)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let radius = 0.2;
            let center = Vec3::new(a as f64+0.9+rand_f64(), radius, b as f64+0.9+rand_f64());
            let offset = Vec3::new(4.0, radius, 0.0);
            if (center - offset).length() > 0.9 {
                if choose_mat < 0.8 {
                    let lam_rand = || { rand_f64()*rand_f64() };
                    let mat = Lambertian::rc(ConstantTexture::rc(Vec3::new(lam_rand(), lam_rand(), lam_rand())));
                    let center1 = center + Vec3::new(0.0, rand_f64() * 0.15, 0.0);
                    result.add_hitable(Sphere::boxed_moving(center, center1, 0.0, 1.0, radius, mat));
                } else if choose_mat < 0.95 {
                    let met_rand = || (1.0 + rand_f64()) * 0.5;
                    let mat = Metal::rc(
                        ConstantTexture::rc(Vec3::new(met_rand(), met_rand(), met_rand())), rand_f64() * 0.5);
                    result.add_hitable(Sphere::boxed(center, radius, mat));
                } else {
                    let mat = Dielectric::rc(1.5);
                    result.add_hitable(Sphere::boxed(center, radius, mat));
                }
            }
        }
    }

    (result, camera, true)
}

pub fn scene_two_spheres(nx: u32, ny: u32) -> (Box<Hitable>, Camera, bool) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let noise: Rc<Material> = Lambertian::rc(NoiseTexture::rc(2.0));
    let earth: Rc<Material> = Lambertian::rc(ImageTexture::rc(Path::new("map.png")));
    let mut objs: Vec<Box<Hitable>> = vec![
        Sphere::boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::clone(&noise)),
        Sphere::boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, Rc::clone(&earth)),
    ];

    let mut result = Box::new(HitableList::new());
    while let Some(obj) = objs.pop() {
        result.add_hitable(obj);
    }
    (result, camera, true)
}

pub fn scene_simple_light(nx: u32, ny: u32) -> (Box<Hitable>, Camera, bool) {
    let look_from = Vec3::new(13.0, 20.0, 22.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let noise: Rc<Material> = Lambertian::rc(NoiseTexture::rc(4.0));
    let light: Rc<Material> = DiffuseLight::rc(ConstantTexture::rc(Vec3::new(4.0, 4.0, 4.0)));
    let mut objs: Vec<Box<Hitable>> = vec![
        Sphere::boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::clone(&noise)),
        Sphere::boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, Rc::clone(&noise)),
        Sphere::boxed(Vec3::new(0.0, 7.0, 0.0), 2.0, Rc::clone(&light)),
        Rect::xyrect(3.0, 1.0, 5.0, 3.0, -2.0, Rc::clone(&light))
    ];

    let mut result = Box::new(HitableList::new());
    while let Some(obj) = objs.pop() {
        result.add_hitable(obj);
    }
    (result, camera, false)
}

pub fn scene_cornell(nx: u32, ny: u32) -> (Box<Hitable>, Camera, bool) {
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), vfov, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let red: Rc<Material> = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.65, 0.05, 0.05)));
    let white: Rc<Material> = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.73, 0.73, 0.73)));
    let green: Rc<Material> = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.12, 0.45, 0.15)));
    let light: Rc<Material> = DiffuseLight::rc(ConstantTexture::rc(Vec3::new(15.0, 15.0, 15.0)));

    let mut objs: Vec<Box<Hitable>> = vec![
        Rect::yzrect(0.0, 0.0, 555.0, 555.0, 555.0, Rc::clone(&green)),
        Rect::yzrect(0.0, 0.0, 555.0, 555.0, 0.0, Rc::clone(&red)),
        Rect::xzrect(213.0, 227.0, 343.0, 332.0, 554.0, Rc::clone(&light)),
        Rect::xzrect(0.0, 0.0, 555.0, 555.0, 555.0, Rc::clone(&white)),
        Rect::xzrect(0.0, 0.0, 555.0, 555.0, 1.0, Rc::clone(&white)),
        Rect::xyrect(0.0, 0.0, 555.0, 555.0, 555.0, Rc::clone(&white))
    ];

    let mut result = Box::new(HitableList::new());
    while let Some(obj) = objs.pop() {
        result.add_hitable(obj);
    }
    (result, camera, false)

}