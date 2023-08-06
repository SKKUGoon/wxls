use crate::Cell;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Cursor {
    /// Excel Workbook >> Worksheet >> Range >> Cell
    /// Controls cells and create range
    pub cell: Cell,
}
