use crate::cell::Cell;
use crate::error::WebExcelError;
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

    pub columns: u32,
    pub rows: u32,
    pub cells: u32,
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.cell_start == other.cell_start && self.cell_end == other.cell_end
    }
}

#[wasm_bindgen]
impl Range {
    /// Ensures that `cell_start` is always before `cell_end`
    fn correctify(&mut self) {
        // If `cell_start` is positioned after `cell_end` in any dimension (row or column),
        // their positions are swapped to make the range valid.
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
    pub fn new(start: &Cell, end: &Cell) -> Result<Range, WebExcelError> {
        if start.sheet != end.sheet {
            return Err(WebExcelError::RangeDiffSheetError);
        }

        let mut range = Range {
            cell_start: start.clone(),
            cell_end: end.clone(),
            columns: start.column.abs_diff(end.column) + 1,
            rows: start.row.abs_diff(end.row) + 1,
            cells: (start.column.abs_diff(end.column) + 1) * (start.row.abs_diff(end.row) + 1),
        };

        // Check starting cell and ending cell, re-arragne them if necessary
        range.correctify();

        Ok(range)
    }

    /// Convert the range to a string representation.
    pub fn to_str_address(&self) -> Result<String, WebExcelError> {
        let addr_start = self.cell_start.to_str_address()?;
        let addr_end = self.cell_end.to_str_address()?;

        Ok(format!("{}:{}", addr_start, addr_end))
    }

    /// Check if a cell is within the range.
    pub fn has(&self, target: &Cell) -> bool {
        self.cell_start.row <= target.row
            && self.cell_end.row >= target.row
            && self.cell_start.column <= target.column
            && self.cell_end.column >= target.column
    }

    /// Check if a range is completely within this range.
    pub fn includes(&self, target: &Range) -> bool {
        self.cell_start.row <= target.cell_start.row
            && self.cell_end.row >= target.cell_end.row
            && self.cell_start.column <= target.cell_start.column
            && self.cell_end.column >= target.cell_end.column
    }

    /// Check if this range intersects with another range.
    pub fn intersects(&self, other: &Range) -> Result<bool, WebExcelError> {
        let other_inv_tr = Cell::new(other.cell_start.row, other.cell_end.column, None)?;
        let other_inv_bl = Cell::new(other.cell_end.row, other.cell_start.column, None)?;

        let self_inv_tr = Cell::new(self.cell_start.row, self.cell_end.column, None)?;
        let self_inv_bl = Cell::new(self.cell_end.row, self.cell_start.column, None)?;

        Ok(self.has(&other_inv_bl)
            || self.has(&other_inv_tr)
            || other.has(&self_inv_bl)
            || other.has(&self_inv_tr))
    }

    /// Extract a sub-range of columns from the current range.
    pub fn select_column(
        &self,
        column_start: usize,
        column_end: usize,
    ) -> Result<Range, WebExcelError> {
        let new_start = Cell::new(
            self.cell_start.row,
            self.cell_start.column + column_start as u32,
            self.cell_start.sheet.clone(),
        )?;

        let new_end = Cell::new(
            self.cell_end.row,
            self.cell_start.column + (column_end - 1) as u32,
            self.cell_start.sheet.clone(),
        )?;

        Range::new(&new_start, &new_end)
    }

    /// Extract a sub-range of rows from the current range.
    pub fn select_row(&self, row_start: usize, row_end: usize) -> Result<Range, WebExcelError> {
        let new_start = Cell::new(
            self.cell_start.row + row_start as u32,
            self.cell_start.column,
            self.cell_start.sheet.clone(),
        )?;

        let new_end = Cell::new(
            self.cell_start.row + (row_end - 1) as u32,
            self.cell_end.column,
            self.cell_start.sheet.clone(),
        )?;

        Range::new(&new_start, &new_end)
    }

    /// Create an iterator over columns within the range.
    pub fn iter_col(&self) -> Result<js_sys::Array, WebExcelError> {
        let mut addresses: Vec<String> = Vec::new();

        for i in self.cell_start.column..=self.cell_end.column {
            let column_start = Cell::new(self.cell_start.row, i, self.cell_start.sheet.clone())?;
            let column_end = Cell::new(self.cell_end.row, i, self.cell_start.sheet.clone())?;

            let column = format!(
                "{}:{}",
                column_start.to_str_address()?,
                column_end.to_str_address()?
            );
            addresses.push(column);
        }

        let boxed = Box::new(addresses);
        Ok(boxed.iter().map(JsValue::from).collect::<js_sys::Array>())
    }

    /// Create an iterator over rows within the range.
    pub fn iter_row(&self) -> Result<js_sys::Array, WebExcelError> {
        let mut addresses: Vec<String> = Vec::new();

        for i in self.cell_start.row..=self.cell_end.row {
            let row_start = Cell::new(i, self.cell_start.column, self.cell_start.sheet.clone())?;
            let row_end = Cell::new(i, self.cell_end.column, self.cell_start.sheet.clone())?;

            let row = format!(
                "{}:{}",
                row_start.to_str_address()?,
                row_end.to_str_address()?
            );
            addresses.push(row);
        }

        let boxed = Box::new(addresses);
        Ok(boxed.iter().map(JsValue::from).collect::<js_sys::Array>())
    }
}
