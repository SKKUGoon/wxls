use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// Normal `println!` in Rust cannot print status to JS console.
// Use `extern "C"` and log to create console_log macro
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        $crate::util::macros::log(&format_args!($($t)*).to_string())
    };
}
