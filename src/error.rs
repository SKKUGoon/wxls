use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum WebExcelError {
    ParseError,
    OutOfBoundError,
    RelocateError,
}

impl fmt::Display for WebExcelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebExcelError::ParseError => write!(f, "WebExcel parse error"),
            WebExcelError::OutOfBoundError => write!(f, "WebExcel address out of bound error"),
            WebExcelError::RelocateError => write!(f, "WebExcel relocate error"),
        }
    }
}

impl std::error::Error for WebExcelError {}
