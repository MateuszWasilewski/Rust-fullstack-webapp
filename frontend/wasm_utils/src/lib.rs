mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        // Use `js_namespace` here to bind `console.log(..)` instead of just
        // `log(..)`
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

#[cfg(debug_assertions)]
pub fn log(s: &str) {
    wasm::log(s);
}

#[cfg(not(debug_assertions))]
pub fn log(_: &str) {}
