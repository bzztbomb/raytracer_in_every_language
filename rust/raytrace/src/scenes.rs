use vec3::Vec3;
use hitable::*;
use material::*;
use camera::*;
use rt_rand::*;

pub fn simple_scene(nx: u32, ny: u32) -> (Box<Hitable>, Camera) {
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 0.0);

    let objs: Vec<Box<Hitable>> = vec![
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

pub fn scene_random(nx: u32, ny: u32) -> (Box<Hitable>, Camera) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.01;
    let camera = Camera::new(&look_from, &look_at, &Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);

    let mut result = Box::new(HitableList::new());
    result.add_hitable(Sphere::boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::rc(Vec3::new(0.5, 0.5, 0.5))));
	result.add_hitable(Sphere::boxed(Vec3::new(0.0,1.0,0.0), 1.0, Dielectric::rc(1.5)));
	result.add_hitable(Sphere::boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::rc(Vec3::new(0.4, 0.2, 0.1))));
	result.add_hitable(Sphere::boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, Metal::rc(Vec3::new(0.7, 0.6, 0.5), 0.0)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let radius = 0.2;
            let center = Vec3::new(a as f64+0.9+rand_f64(), radius, b as f64+0.9+rand_f64());
            let offset = Vec3::new(4.0, radius, 0.0);
            if (center - offset).length() > 0.9 {
                if choose_mat < 0.8 {
                    let lam_rand = || { rand_f64()*rand_f64() };
                    let mat = Lambertian::rc(Vec3::new(lam_rand(), lam_rand(), lam_rand()));
                    let center1 = center + Vec3::new(0.0, rand_f64() * 0.15, 0.0);
                    result.add_hitable(Sphere::boxed_moving(center, center1, 0.0, 1.0, radius, mat));
                } else if choose_mat < 0.95 {
                    let met_rand = || (1.0 + rand_f64()) * 0.5;
                    let mat = Metal::rc(
                        Vec3::new(met_rand(), met_rand(), met_rand()), rand_f64() * 0.5);
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

