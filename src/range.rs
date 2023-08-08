use crate::Cell;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Range {
    sheet: &'static str,
    address: Cell,
}

impl Range {
    pub fn new(sheet: &'static str, rc_address: Cell) -> Self {
        Range {
            sheet,
            address: rc_address,
        }
    }

    pub fn to_str_address(&self) -> String {
        format!("{}!{}", self.sheet, self.address.to_str_address())
    }

    // pub fn envelope<T>(matrix: Vec<Vec<T>>) {

    // }
}
