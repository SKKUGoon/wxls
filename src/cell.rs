use crate::AddressRC;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    sheet: &'static str,
    address: &'static str,
    rc_address: AddressRC,
}

// impl Cell {
//     pub fn new(rc_address: AddressRC) -> Self {
//         Cell { rc_address }
//     }
// }
