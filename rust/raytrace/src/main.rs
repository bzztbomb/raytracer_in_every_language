extern crate raytrace;

use raytrace::renderer::Renderer;

const NX: u32 = 400;
const NY: u32 = 200;
const NS: u32 = 10;

fn main() {
    let renderer = Renderer::new(NX, NY, NS);

    // let debug_r = camera.get_ray(131.0 / NX as f64, 98.0 / NY as f64);
    // println!("ray: {:?}\n", debug_r);
    // let p = color(&debug_r, &scene, 0);
    // println!("{:?}\n", p);

    println!("P3\n{} {}\n255", NX, NY);
    for j in (0..NY).rev() {
        for i in 0..NX {
            let c = renderer.pixel_color(i, j);
            println!("{} {} {}", c.x as i32, c.y as i32, c.z as i32);
        }
    }
}

