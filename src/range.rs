use crate::error::WebExcelError;
use crate::{cell::Cell, error};
use std::mem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum RangeAnchorStyle {
    Row = "row",
    Column = "column",
    Start = "start",
    End = "end",
    All = "all",
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct Range {
    #[wasm_bindgen(getter_with_clone)]
    pub cell_start: Cell,

    #[wasm_bindgen(getter_with_clone)]
    pub cell_end: Cell,
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.cell_start == other.cell_start && self.cell_end == other.cell_end
    }
}

#[wasm_bindgen]
impl Range {
    /// Ensures that `cell_start` is positioned before `cell_end` in terms of row and column.
    /// If `cell_start` is positioned after `cell_end` in any dimension (row or column),
    /// their positions are swapped to make the range valid.
    fn correctify(&mut self) {
        let (mut new_start, mut new_end) = (self.cell_start.clone(), self.cell_end.clone());

        // If the start cell's column is greater than the end cell's column, swap them.
        if self.cell_start.column > self.cell_end.column {
            mem::swap(&mut new_start.column, &mut new_end.column);
        }

        // If the start cell's row is greater than the end cell's row, swap them.
        if self.cell_start.row > self.cell_end.row {
            mem::swap(&mut new_start.row, &mut new_end.row);
        }

        self.cell_start = new_start;
        self.cell_end = new_end;
    }

    #[wasm_bindgen(constructor)]
    pub fn new(start: &Cell, end: &Cell) -> Result<Range, error::WebExcelError> {
        if start.sheet != end.sheet {
            return Err(WebExcelError::RangeDiffSheetError);
        }

        let mut range = Range {
            cell_start: start.clone(),
            cell_end: end.clone(),
        };

        // Check starting cell and ending cell, re-arragne them if necessary
        range.correctify();

        Ok(range)
    }

    pub fn to_str_address(&self) -> Result<String, error::WebExcelError> {
        let addr_start = self.cell_start.to_str_address()?;
        let addr_end = self.cell_end.to_str_address()?;

        Ok(format!("{}:{}", addr_start, addr_end))
    }

    pub fn has(&self, target: &Cell) -> bool {
        self.cell_start.row <= target.row
            && self.cell_end.row >= target.row
            && self.cell_start.column <= target.column
            && self.cell_end.column >= target.column
    }

    pub fn iter_col() {}

    pub fn iter_row() {}
}
