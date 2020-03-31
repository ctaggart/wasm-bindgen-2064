mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn greet() {
    log("hello node");
    // log(&version);
    // log(&ts.version());
    log(&ts.getNodeMajorVersion());
}


// https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


// #[wasm_bindgen]
// extern "C" {
//     type Console;

//     #[wasm_bindgen(js_namespace = console)]
//     static console: Console;

//     #[wasm_bindgen(method)]
//     fn log(this: &Console, s: &str);
// }

// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/module.html

#[wasm_bindgen(module = "typescript")]
extern {
    
    #[wasm_bindgen(js_name = default)]
    type Ts;
    
    #[wasm_bindgen(js_name = default)]
    static ts: Ts;
    // static version: String;

    #[wasm_bindgen(method)]
    fn getNodeMajorVersion(this: &Ts) -> String;
}


// #[wasm_bindgen(module = "typescript")]
// extern {
//     type Ts;

//     static ts: Ts;

//     // fn version(this: &Ts) -> String;
//     #[wasm_bindgen(method)]
//     fn getNodeMajorVersion(this: &Ts) -> String;
// }

// https://github.com/rustwasm/wasm-bindgen/pull/1106