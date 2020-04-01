use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() {
    log("hello node");
    if let Some(major_node_version) = getNodeMajorVersion() {
        log(&format!("Node major version: {}", &major_node_version));
    }
    log(&format!("TypeScript version: {}", &version.to_string()));
}

// https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/module.html
#[wasm_bindgen(module = "typescript")]
extern {
    fn getNodeMajorVersion() -> Option<u32>;
    static version: String;
}