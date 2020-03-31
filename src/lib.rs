use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() {
    log("hello node");
    // log(&ts.getNodeMajorVersion());
    log(&getNodeMajorVersion());
    // log(&version);
}

// https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/module.html
// default pr https://github.com/rustwasm/wasm-bindgen/pull/1106/files
#[wasm_bindgen(module = "typescript")]
extern {
    fn getNodeMajorVersion() -> String;

    // #[wasm_bindgen(js_name = default)]
    // type Ts;
    // static ts: Ts;

    // #[wasm_bindgen(method)]
    // fn getNodeMajorVersion(this: &Ts) -> String;

    // static version: String;
}