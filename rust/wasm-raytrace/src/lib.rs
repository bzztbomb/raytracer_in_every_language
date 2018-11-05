extern crate cfg_if;
extern crate wasm_bindgen;
extern crate raytrace;

use raytrace::renderer::Renderer;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-raytrace!");
}

#[wasm_bindgen]
pub struct WasmRendererWrapper {
    renderer: Renderer,
}

#[wasm_bindgen]
impl WasmRendererWrapper {
    pub fn new(nx: u32, ny: u32, ns: u32) -> WasmRendererWrapper {
        WasmRendererWrapper {
            renderer: Renderer::new(nx, ny, ns)
        }
    }

    pub fn pixel_color(&self, i: u32, j: u32, ret: &mut [u8]) {
        let c = self.renderer.pixel_color(i, j);
        ret[0] = c.x as u8;
        ret[1] = c.y as u8;
        ret[2] = c.z as u8;
    }
}




