use crate::{console_log, Cell};
use std::cmp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Range {
    /// A12:B45
    /// ^^^
    /// Define as Start
    pub row_start: u32,
    pub column_start: u32,

    #[wasm_bindgen(getter_with_clone)]
    pub sheet_start: Option<String>,

    /// A12:B45
    ///     ^^^
    /// Define as End
    pub row_end: u32,
    pub column_end: u32,

    #[wasm_bindgen(getter_with_clone)]
    pub sheet_end: Option<String>,
}

// impl PartialEq for Range {
//     fn eq(&self, other: &Self) -> bool {
//         self.address == other.address
//     }
// }

#[wasm_bindgen]
impl Range {
    #[wasm_bindgen(constructor)]
    pub fn new(start: &Cell, end: &Cell) -> Result<Range, String> {
        // Root out the case where it has wrong sheets.
        // If both cell has sheet, it should be the same.
        // If only the ending cell has sheet, it's erroneous
        // If only the starting cell has sheet, warn user that it cannot be used with `autofill` method of excel.
        match (&start.sheet, &end.sheet) {
            (Some(start_sheet), Some(end_sheet)) => {
                if start_sheet != end_sheet {
                    let msg = "[wxls] single range cannot have different sheets";
                    console_log!("{}", msg);
                    return Err(msg.to_string());
                }
            }
            (Some(_), None) => {
                let msg = "[wxls] `end` sheet not given. \
                 reommend not using it as autofill";
                console_log!("{}", msg);
            }
            (None, Some(_)) => {
                let msg = "[wxls] if `end` Cell has sheet, `start` Cell's sheet is required";
                console_log!("{}", msg);
                return Err(msg.to_string());
            }
            _ => {}
        };

        Ok(Range {
            row_start: start.row,
            column_start: start.column,
            sheet_start: start.sheet.clone(),

            row_end: end.row,
            column_end: end.column,
            sheet_end: end.sheet.clone(),
        })
    }

    pub fn to_str_address(&self) -> String {
        let cell_start = Cell::new(self.row_start, self.row_end, self.sheet_start.clone()).unwrap();
        let cell_end = Cell::new(self.row_end, self.column_end, self.sheet_end.clone()).unwrap();

        format!(
            "{}:{}",
            cell_start.to_str_address(),
            cell_end.to_str_address()
        )
    }

    pub fn has(&self, target: &Cell) -> bool {
        // sheet_start and sheet_end is the same.
        // Guaranteed by `self.new`
        let sheet_condition = match (&target.sheet, &self.sheet_start) {
            (Some(target_sheet), Some(my_sheet)) => target_sheet == my_sheet,
            (None, None) => true,
            _ => false,
        };

        let index_condition = target.row >= self.row_start
            && target.row <= self.row_end
            && target.column >= self.column_start
            && target.column <= self.column_end;

        sheet_condition && index_condition
    }

    pub fn new_include(&mut self, target: &Cell) -> Result<Range, String> {
        // If sheet is different, cannot include new target
        match (&self.sheet_start, &target.sheet) {
            (Some(my_sheet), Some(target_sheet)) => {
                if my_sheet != target_sheet {
                    let msg = "[wxls] cannot create range with different sheets";
                    console_log!("{}", msg);
                    return Err(msg.to_string());
                }
            }
            (None, Some(_)) => {
                let msg = "[wxls] cannot create range with different sheets";
                console_log!("{}", msg);
                return Err(msg.to_string());
            }
            _ => {}
        };

        if self.has(target) {
            Ok(self.clone())
        } else {
            // Sheet information is guaranteed
            Ok(Range {
                row_start: cmp::min(target.row, self.row_start),
                column_start: cmp::min(target.column, self.column_start),
                sheet_start: self.sheet_start.clone(),
                row_end: cmp::max(target.row, self.row_end),
                column_end: cmp::max(target.column, self.column_end),
                sheet_end: self.sheet_end.clone(),
            })
        }
    }

    // pub fn envelope<T>(matrix: Vec<Vec<T>>) {

    // }
}
