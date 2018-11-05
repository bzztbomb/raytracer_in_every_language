/* tslint:disable */
import * as wasm from './wasm_raytrace_bg';

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_alert_0608be5b7b4705e0(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    alert(varg0);
}
/**
* @returns {void}
*/
export function greet() {
    return wasm.greet();
}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    return [ptr, arg.length];
}

function freeWasmRendererWrapper(ptr) {

    wasm.__wbg_wasmrendererwrapper_free(ptr);
}
/**
*/
export class WasmRendererWrapper {

    static __wrap(ptr) {
        const obj = Object.create(WasmRendererWrapper.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeWasmRendererWrapper(ptr);
    }

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @param {number} arg2
    * @returns {WasmRendererWrapper}
    */
    static new(arg0, arg1, arg2) {
        return WasmRendererWrapper.__wrap(wasm.wasmrendererwrapper_new(arg0, arg1, arg2));
    }
    /**
    * @param {number} arg0
    * @param {number} arg1
    * @param {Uint8Array} arg2
    * @returns {void}
    */
    pixel_color(arg0, arg1, arg2) {
        const [ptr2, len2] = passArray8ToWasm(arg2);
        try {
            return wasm.wasmrendererwrapper_pixel_color(this.ptr, arg0, arg1, ptr2, len2);

        } finally {
            arg2.set(getUint8Memory().subarray(ptr2 / 1, ptr2 / 1 + len2));
            wasm.__wbindgen_free(ptr2, len2 * 1);

        }

    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

export function __wbindgen_Math_tan(x) { return Math.tan(x); }

