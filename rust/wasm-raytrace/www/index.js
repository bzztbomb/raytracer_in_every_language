import { WasmRendererWrapper } from "wasm-raytrace";

const canvas = document.getElementById("raytrace-canvas");
canvas.width = 400;
canvas.height = 200;

const pixelRet = new Uint8Array(3);

console.log("Test");
const renderer = WasmRendererWrapper.new(canvas.width, canvas.height, 100);
const ctx = canvas.getContext('2d');
for (var j = 0; j < canvas.height; j++) {
  for (var i = 0; i < canvas.width; i++) {
    renderer.pixel_color(i, canvas.height - j - 1, pixelRet);
    ctx.fillStyle = 'rgb(' + pixelRet.join(',') + ')';
    ctx.fillRect(i, j, 1, 1);
  }
}
