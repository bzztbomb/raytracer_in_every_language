use std::sync::Arc;
use std::path::Path;

use vec3::Vec3;
use hitable::*;
use material::*;
use camera::*;
use rt_rand::*;
use texture::*;
use aabb::Aabb;
use constant_medium::ConstantMedium;

pub fn simple_scene(nx: u32, ny: u32) -> (HitablePtr, Camera, bool) {
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.1;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 0.0);

    let objs: Vec<HitablePtr> = vec![
        Sphere::hitable_ptr(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::rc(ConstantTexture::rc(Vec3::new(0.5, 0.5, 0.5)))),
        Sphere::hitable_ptr(Vec3::new(0.0, -100.5, 0.0), 100.0, Lambertian::rc(ConstantTexture::rc(Vec3::new(0.0, 1.0, 0.0)))),
        Sphere::hitable_ptr(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::rc(ConstantTexture::rc(Vec3::new(0.8, 0.6, 0.2)), 0.03)),
        Sphere::hitable_ptr(Vec3::new(-1.0, 0.0, -1.0), 0.5, Metal::rc(ConstantTexture::rc(Vec3::new(0.8, 0.8, 0.8)), 1.0)),
        // Sphere::hitable_ptr(Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::rc(1.5)),
        // Sphere::hitable_ptr(Vec3::new(-1.0, 0.0, -1.0), -0.45, Dielectric::rc(1.5)),
    ];

    let result = Arc::new(Bvh::new(objs, 0.0, 1.0));
    // let mut result = Box::new(HitableList::new());
    // while let Some(obj) = objs.pop() {
    //     result.add_hitable(obj);
    // }
    (result, camera, true)
}

pub fn scene_random(nx: u32, ny: u32) -> (HitablePtr, Camera, bool) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.01;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let mut result_ptr = Arc::new(HitableList::new());
    {
        let result = Arc::get_mut(&mut result_ptr).unwrap();
        result.add_hitable(Sphere::hitable_ptr(Vec3::new(0.0, -1000.0, 0.0), 1000.0,
            Lambertian::rc(CheckerTexture::rc(
                ConstantTexture::rc(Vec3::new(0.2, 0.3, 0.1)),
                ConstantTexture::rc(Vec3::new(0.9, 0.9, 0.9))
            ))));
        result.add_hitable(Sphere::hitable_ptr(Vec3::new(0.0,1.0,0.0), 1.0, Dielectric::rc(1.5)));
        result.add_hitable(Sphere::hitable_ptr(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::rc(ConstantTexture::rc(Vec3::new(0.4, 0.2, 0.1)))));
        result.add_hitable(Sphere::hitable_ptr(Vec3::new(4.0, 1.0, 0.0), 1.0, Metal::rc(ConstantTexture::rc(Vec3::new(0.7, 0.6, 0.5)), 0.0)));

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
                        result.add_hitable(Sphere::hitable_ptr_moving(center, center1, 0.0, 1.0, radius, mat));
                    } else if choose_mat < 0.95 {
                        let met_rand = || (1.0 + rand_f64()) * 0.5;
                        let mat = Metal::rc(
                            ConstantTexture::rc(Vec3::new(met_rand(), met_rand(), met_rand())), rand_f64() * 0.5);
                        result.add_hitable(Sphere::hitable_ptr(center, radius, mat));
                    } else {
                        let mat = Dielectric::rc(1.5);
                        result.add_hitable(Sphere::hitable_ptr(center, radius, mat));
                    }
                }
            }
        }
    }

    (result_ptr, camera, true)
}

pub fn scene_two_spheres(nx: u32, ny: u32) -> (HitablePtr, Camera, bool) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let noise: MaterialPtr = Lambertian::rc(NoiseTexture::rc(2.0));
    let earth: MaterialPtr = Lambertian::rc(ImageTexture::rc(Path::new("map.png")));
    let mut objs: Vec<HitablePtr> = vec![
        Sphere::hitable_ptr(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&noise)),
        Sphere::hitable_ptr(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::clone(&earth)),
    ];

    let mut result_ptr = Arc::new(HitableList::new());
    {
        let result = Arc::get_mut(&mut result_ptr).unwrap();
        while let Some(obj) = objs.pop() {
            result.add_hitable(obj);
        }
    }
    (result_ptr, camera, true)
}

pub fn scene_simple_light(nx: u32, ny: u32) -> (HitablePtr, Camera, bool) {
    let look_from = Vec3::new(13.0, 20.0, 22.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let noise: MaterialPtr = Lambertian::rc(NoiseTexture::rc(4.0));
    let light: MaterialPtr = DiffuseLight::rc(ConstantTexture::rc(Vec3::new(4.0, 4.0, 4.0)));
    let mut objs: Vec<HitablePtr> = vec![
        Sphere::hitable_ptr(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&noise)),
        Sphere::hitable_ptr(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::clone(&noise)),
        Sphere::hitable_ptr(Vec3::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&light)),
        Rect::xyrect(3.0, 1.0, 5.0, 3.0, -2.0, Arc::clone(&light))
    ];


    let mut result_ptr = Arc::new(HitableList::new());
    {
        let result = Arc::get_mut(&mut result_ptr).unwrap();
        while let Some(obj) = objs.pop() {
            result.add_hitable(obj);
        }
    }
    (result_ptr, camera, false)
}

pub fn scene_cornell(nx: u32, ny: u32) -> (HitablePtr, Camera, bool) {
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), vfov, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let red: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.65, 0.05, 0.05)));
    let white: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.73, 0.73, 0.73)));
    let green: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.12, 0.45, 0.15)));
    let light: MaterialPtr = DiffuseLight::rc(ConstantTexture::rc(Vec3::new(15.0, 15.0, 15.0)));

    let mut objs: Vec<HitablePtr> = vec![
        FlipNormals::hitable_ptr(Rect::yzrect(0.0, 0.0, 555.0, 555.0, 555.0, Arc::clone(&green))),
        Rect::yzrect(0.0, 0.0, 555.0, 555.0, 0.0, Arc::clone(&red)),
        Rect::xzrect(213.0, 227.0, 343.0, 332.0, 554.0, Arc::clone(&light)),
        FlipNormals::hitable_ptr(Rect::xzrect(0.0, 0.0, 555.0, 555.0, 555.0, Arc::clone(&white))),
        Rect::xzrect(0.0, 0.0, 555.0, 555.0, 1.0, Arc::clone(&white)),
        FlipNormals::hitable_ptr(Rect::xyrect(0.0, 0.0, 555.0, 555.0, 555.0, Arc::clone(&white))),
        Translate::hitable_ptr(RotateY::hitable_ptr(AabbBox::hitable_ptr(Aabb::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0)), Arc::clone(&white)), -18.0), Vec3::new(130.0, 0.0, 65.0)),
        Translate::hitable_ptr(RotateY::hitable_ptr(AabbBox::hitable_ptr(Aabb::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0)), Arc::clone(&white)), 15.0), Vec3::new(265.0, 0.0, 295.0)),
    ];

    let mut result_ptr = Arc::new(HitableList::new());
    {
        let result = Arc::get_mut(&mut result_ptr).unwrap();
        while let Some(obj) = objs.pop() {
            result.add_hitable(obj);
        }
    }
    (result_ptr, camera, false)
}

pub fn scene_cornell_volumes(nx: u32, ny: u32) -> (HitablePtr, Camera, bool) {
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), vfov, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let red: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.65, 0.05, 0.05)));
    let white: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.73, 0.73, 0.73)));
    let green: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.12, 0.45, 0.15)));
    let light: MaterialPtr = DiffuseLight::rc(ConstantTexture::rc(Vec3::new(7.0, 7.0, 7.0)));

    let b1 = Translate::hitable_ptr(RotateY::hitable_ptr(AabbBox::hitable_ptr(Aabb::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0)), Arc::clone(&white)), -18.0), Vec3::new(130.0, 0.0, 65.0));
    let b2 = Translate::hitable_ptr(RotateY::hitable_ptr(AabbBox::hitable_ptr(Aabb::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0)), Arc::clone(&white)), 15.0), Vec3::new(265.0, 0.0, 295.0));

    let mut objs: Vec<HitablePtr> = vec![
        FlipNormals::hitable_ptr(Rect::yzrect(0.0, 0.0, 555.0, 555.0, 555.0, Arc::clone(&green))),
        Rect::yzrect(0.0, 0.0, 555.0, 555.0, 0.0, Arc::clone(&red)),
        Rect::xzrect(113.0, 127.0, 443.0, 432.0, 554.0, Arc::clone(&light)),
        FlipNormals::hitable_ptr(Rect::xzrect(0.0, 0.0, 555.0, 555.0, 555.0, Arc::clone(&white))),
        Rect::xzrect(0.0, 0.0, 555.0, 555.0, 1.0, Arc::clone(&white)),
        FlipNormals::hitable_ptr(Rect::xyrect(0.0, 0.0, 555.0, 555.0, 555.0, Arc::clone(&white))),
        ConstantMedium::hitable_ptr(&b1, 0.01, ConstantTexture::rc(Vec3::new(1.0, 1.0, 1.0))),
        ConstantMedium::hitable_ptr(&b2, 0.01, ConstantTexture::rc(Vec3::new(0.0, 0.0, 0.0)))
    ];

    let mut result_ptr = Arc::new(HitableList::new());
    {
        let result = Arc::get_mut(&mut result_ptr).unwrap();
        while let Some(obj) = objs.pop() {
            result.add_hitable(obj);
        }
    }
    (result_ptr, camera, false)
}

pub fn scene_final(nx: u32, ny: u32) -> (HitablePtr, Camera, bool) {
	let look_from = Vec3::new(478.0, 278.0, -600.0);
	let look_at = Vec3::new(278.0, 278.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.0;
	let vfov = 40.0;
	let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0,1.0,0.0), vfov, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let ground: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.48, 0.83, 0.53)));

    let mut result_ptr = Arc::new(HitableList::new());
    if let Some(result) = Arc::get_mut(&mut result_ptr) {
        // Ground
        let mut box_objects: Vec<HitablePtr> = Vec::new();
        let num_boxes_per_side = 20;
        for i in 0..num_boxes_per_side {
            for j in 0..num_boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = 100.0 * (rand_f64() + 0.01);
                let z1 = z0 + w;
                box_objects.push(AabbBox::hitable_ptr(Aabb::new(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1)), ground.clone()));
            }
        }
        let ground: HitablePtr = Arc::new(Bvh::new(box_objects, 0.0, 1.0));
        result.add_hitable(ground);

        let light = DiffuseLight::rc(ConstantTexture::rc(Vec3::new(7.0, 7.0, 7.0)));
        result.add_hitable(Rect::xzrect(123.0, 147.0, 423.0, 412.0, 554.0, light.clone()));

        let center = Vec3::new(400.0, 400.0, 200.0);
        let sphere_mat: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.7, 0.3, 0.1)));
        result.add_hitable(Sphere::hitable_ptr_moving(center, center + Vec3::new(30.0, 0.0, 0.0), 0.0, 1.0, 50.0, sphere_mat));

        let dielectric = Dielectric::rc(1.5);
        result.add_hitable(Sphere::hitable_ptr(Vec3::new(260.0, 150.0, 45.0), 50.0, dielectric.clone()));
        result.add_hitable(Sphere::hitable_ptr(Vec3::new(0.0, 150.0, 145.0), 50.0, Metal::rc(ConstantTexture::rc(Vec3::new(0.8, 0.8, 0.9)), 10.0)));

        let boundary: HitablePtr = Sphere::hitable_ptr(Vec3::new(360.0, 150.0, 145.0), 70.0, dielectric.clone());
        result.add_hitable(boundary.clone());
        result.add_hitable(ConstantMedium::hitable_ptr(&boundary, 0.2, ConstantTexture::rc(Vec3::new(0.2, 0.4, 0.9))));

        let room_haze: HitablePtr = Sphere::hitable_ptr(Vec3::new(0.0, 0.0, 0.0), 5000.0, dielectric.clone());
        result.add_hitable(ConstantMedium::hitable_ptr(&room_haze, 0.0001, ConstantTexture::rc(Vec3::new(1.0, 1.0, 1.0))));

        let earth: MaterialPtr = Lambertian::rc(ImageTexture::rc(Path::new("map.png")));
        result.add_hitable(Sphere::hitable_ptr(Vec3::new(400.0, 200.0, 400.0), 100.0, earth));

        result.add_hitable(Sphere::hitable_ptr(Vec3::new(220.0, 280.0, 300.0), 80.0, Lambertian::rc(NoiseTexture::rc(0.1))));

        let white: MaterialPtr = Lambertian::rc(ConstantTexture::rc(Vec3::new(0.73, 0.73, 0.73)));
        let num_spheres = 1000;
        let mut sphere_objects: Vec<HitablePtr> = Vec::new();
        for _ in 0..num_spheres {
            let center = Vec3::new(rand_f64() * 165.0, rand_f64() * 165.0, rand_f64() * 165.0);
            sphere_objects.push(Sphere::hitable_ptr(center, 10.0, white.clone()));
        }
        let cube: HitablePtr = Arc::new(Bvh::new(sphere_objects, 0.0, 1.0));
        let xformed_cube = Translate::hitable_ptr(RotateY::hitable_ptr(cube, 15.0), Vec3::new(-100.0, 270.0, 395.0));
        result.add_hitable(xformed_cube);
    }

    (result_ptr, camera, false)
}