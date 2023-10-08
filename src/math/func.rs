use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn placeholder() {}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FuncBuilder {
    #[wasm_bindgen(getter_with_clone)]
    pub operations: Box<[i32]>,

    #[wasm_bindgen(getter_with_clone)]
    pub sheet_dependency: Box<[js_sys::JsString]>,
}

#[wasm_bindgen]
pub struct Symbol {
    pub key: i32,

    #[wasm_bindgen(getter_with_clone)]
    pub symbol: String,

    #[wasm_bindgen(getter_with_clone)]
    pub prefix: String,

    #[wasm_bindgen(getter_with_clone)]
    pub suffix: String,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Arithmetic {
    Addition = 0,
    Subtraction = 1,
    Multiplication = 2,
    Division = 3,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum FuncFmt {
    Bracket = 0,
    FuncOut = 1,
}

// #[wasm_bindgen]
// pub enum
