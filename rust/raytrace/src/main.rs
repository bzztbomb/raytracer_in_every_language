extern crate raytrace;
extern crate rayon;

use rayon::prelude::*;
use raytrace::renderer::Renderer;
use std::fmt;

const NX: u32 = 1000;
const NY: u32 = 1000;
const NS: u32 = 5000;

struct WorkChunk {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    result: Vec<f64>,
}

impl WorkChunk {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> WorkChunk {
        WorkChunk {
            x,
            y,
            w,
            h,
            result: vec![0.0; w*h*3]
        }
    }
}

impl fmt::Debug for WorkChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WorkChunk {} {} {} {} {}\n", self.x, self.y, self.w, self.h, self.result[2])
    }
}

struct Chunker {
    width: usize,
    height: usize,
    chunk_size: usize,
    chunk_x: usize,
    chunk_y: usize,
}

impl Chunker {
    pub fn new(width: usize, height: usize, chunk_size: usize) -> Chunker {
        Chunker {
            width,
            height,
            chunk_size,
            chunk_x: 0,
            chunk_y: 0
        }
    }
}

impl Iterator for Chunker {
    type Item = WorkChunk;

    fn next(&mut self) -> Option<WorkChunk> {
        if self.chunk_y * self.chunk_size < self.height {
            let x_size = (self.width - self.chunk_size * self.chunk_x).min(self.chunk_size);
            let y_size = (self.height - self.chunk_size * self.chunk_y).min(self.chunk_size);
            let ret = WorkChunk::new(self.chunk_x * self.chunk_size, self.chunk_y * self.chunk_size, x_size, y_size);
            self.chunk_x += 1;
            if self.chunk_x * self.chunk_size > self.width {
                self.chunk_y += 1;
                self.chunk_x = 0;
            }
            Some(ret)
        } else {
            None
        }
    }
}

fn main() {
    let renderer = Renderer::new(NX, NY, NS);
    let chunker = Chunker::new(NX as usize, NY as usize, 16);
    let results: Vec<WorkChunk> = chunker.collect::<Vec<WorkChunk>>().into_par_iter().update(|work| {
        for y in 0..work.h {
            for x in 0..work.w {
                let c = renderer.pixel_color((x+work.x) as u32, (y+work.y) as u32);
                let offset = (y * work.w + x) * 3;
                work.result[offset] = c.x;
                work.result[offset+1] = c.y;
                work.result[offset+2] = c.z;
            }
        }
    }).collect();
    let mut full_image = vec![0.0; (NX*NY*3) as usize];
    for work in results {
        let mut w_offset = 0;
        for y in 0..work.h {
            let mut offset = ((((NY-1) as usize - (y+work.y)) * NX as usize) + work.x) * 3;
            for _ in 0..work.w {
                for _ in 0..3 {
                    full_image[offset] = work.result[w_offset];
                    offset += 1;
                    w_offset += 1;
                }
            }
        }
    }
    println!("P3\n{} {}\n255", NX, NY);
    for pixel in full_image.chunks(3) {
        println!("{} {} {}", pixel[0] as i32, pixel[1] as i32, pixel[2] as i32);
    }
}