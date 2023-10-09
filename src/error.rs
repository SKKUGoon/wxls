use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum WebExcelError {
    ParseError,
    OutOfBoundError,
    RelocateError,
    RangeDiffSheetError,
}

impl fmt::Display for WebExcelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebExcelError::ParseError => write!(f, "WebExcel parse error"),
            WebExcelError::OutOfBoundError => write!(f, "WebExcel address out of bound error"),
            WebExcelError::RelocateError => write!(f, "WebExcel relocate error"),
            WebExcelError::RangeDiffSheetError => write!(
                f,
                "WebExcel cannot create range with two different sheet for cells"
            ),
        }
    }
}

impl std::error::Error for WebExcelError {}

impl Into<JsValue> for WebExcelError {
    // Need `Into<JsValue>`
    fn into(self) -> JsValue {
        // Convert the error enum into a string representation
        let error_message = format!("{}", self);
        JsValue::from_str(&error_message)
    }
}
