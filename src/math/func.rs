use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FunctionBuilder {
    #[wasm_bindgen(getter_with_clone)]
    pub function: String,

    pub value: i64,

    #[wasm_bindgen(getter_with_clone)]
    pub sheet_dependency: Box<[js_sys::JsString]>,
}

impl PartialEq for FunctionBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.function == other.function
    }
}

// #[wasm_bindgen]
// impl FunctionBuilder {
//     #[wasm_bindgen(constructor)]
//     pub fn new<T>(starting_value: T) {}
// }
