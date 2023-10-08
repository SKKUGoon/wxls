use crate::util::cell_handle::*;
use crate::{console_log, error};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// Enum representing the style of anchoring for Excel cells.
/// - `Row`: Anchors the row, e.g., A1 becomes A$1.
/// - `Column`: Anchors the column, e.g., A1 becomes $A1.
/// - `All`: Anchors both row and column, e.g., A1 becomes $A$1.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum AnchorStyle {
    Row = "row",
    Column = "column",
    All = "all",
}

/// Represents a cell in an Excel sheet.
/// Cells are the basic building blocks in the wxls (WASM Excel) class structure.
/// The hierarchy goes like `Cell` -> `Range` -> `ExcelFunc`.
/// Both `row` and `column` are mandatory, but `sheet` is optional.
#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct Cell {
    /// Row start from index 0. For example, 0 => Excel Row 1, 1 => Excel Row 2,
    pub row: u32,
    /// Column start from index 0. For example, 0 => A, 1 => B, ...
    pub column: u32,
    /// Optional sheet name. If `None`, the cell is considered local.
    /// It's recommended to use the `sheet` option as Microsoft Office JS API's autofill functions may prune `None` sheet cells.
    #[wasm_bindgen(getter_with_clone)]
    // WASM package should support clone. Make Clone with (`getter_with_clone`)
    pub sheet: Option<String>,

    /// Indicates if the row is anchored. When true, A1 becomes A$1.
    pub fixed_row: bool,
    /// Indicates if the column is anchored. When true, A1 becomes $A1.
    pub fixed_column: bool,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        let same_sheet = match (&self.sheet, &other.sheet) {
            (Some(my), Some(oth)) => my == oth,
            (None, None) => true,
            _ => false,
        };

        same_sheet && (self.row == other.row && self.column == other.column)
    }
}

impl FromStr for Cell {
    type Err = error::WebExcelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check whether `s` contains any illegal character

        // Check whether `s` contains character "!". String in front of "!" goes to sheet name
        const SHEET_DIVIDE: &str = "!";

        if s.contains(SHEET_DIVIDE) {
            // Parse address with sheet
            let cell_name_parsed: Vec<&str> = s.split(SHEET_DIVIDE).collect();

            // Create cells
            let mut cell = address_to_r1c1(cell_name_parsed[1])?;

            // Add sheet name
            cell.sheet = Some(cell_name_parsed[0].to_owned());

            Ok(cell)
        } else {
            // Parse address without sheet information
            let mut cell = address_to_r1c1(s)?;

            // Add sheet name
            cell.sheet = None;

            Ok(cell)
        }
    }
}

#[wasm_bindgen]
impl Cell {
    #[wasm_bindgen(constructor)]
    pub fn new(row: u32, column: u32, sheet: Option<String>) -> Result<Cell, String> {
        if true {
            Ok(Cell {
                row,
                column,
                sheet,
                ..Default::default()
            })
        } else {
            let msg = "[wxls] extensive cell creation error".to_string();
            console_log!("{}", msg);
            Err(msg)
        }
    }

    /// TODO: When `trait impls` are supported in `wasm_bindgen`
    /// Change it to std::str::FromStr trait impl
    pub fn from_str_address(data: &str, sheet: Option<String>) -> Result<Cell, String> {
        let chars = data.chars();
        let mut column = 0isize;
        let mut row = 0isize;

        // Cell address `data` contains 2 parts;
        // Row in positive integer(capture with is_ascii_digit),
        //   and Column Starting from A(capture with is_ascii_alphabetic)
        for c in chars {
            if c.is_ascii_digit() {
                row = row * 10 + (c as usize - '0' as usize) as isize;
            } else if c.is_ascii_alphabetic() {
                // If Column A - Z is done, it's changed to AA, AB, AC... until XFD.
                // Subtract  - 'A' ascii number. This act as an offset. A = 0, B = 1
                // If column is not 0, It means that the column notaion has more than 1 letter.
                // Multiply 26 to handle more than 1 letter column.
                column =
                    column * 26 + (c.to_ascii_uppercase() as usize - 'A' as usize) as isize + 1;
            } else {
                let msg = format!(
                    "[wxls] string address conversion error. \
                    Type aside from digit, and alphabets are used {}",
                    data
                );
                console_log!("{}", msg);
                return Err(msg);
            }
        }

        if row - 1 < 0 || column - 1 < 0 {
            let msg = "[wxls] string address returned negative ro and column".to_string();
            console_log!("{}", msg);
            return Err(msg);
        }

        Ok(Cell {
            row: (row - 1) as u32,
            column: (column - 1) as u32,
            sheet,
            ..Default::default()
        })
    }

    /// Attach acquired sheet information to structure
    pub fn attach_sheet(&mut self, s: String) {
        self.sheet = Some(s);
    }

    pub fn to_str_address(&self) -> String {
        let wr = self.row + 1;
        let mut wc = self.column + 1;

        let mut str_column = String::from("");
        str_column = loop {
            let remain = (wc - 1) % 26; // Mod by 26 (total number of alphabet)
            let name_element = char::from_u32(65u32 + remain).unwrap(); // Add 65, Code point for letter 'A' in unicode.

            str_column.push(name_element);
            wc = (wc - remain) / 26;
            if wc == 0 {
                break rc_fix(&str_column, false);
            }
        };

        let str_row = rc_fix(&wr.to_string(), false);

        match &self.sheet {
            Some(my_sheet) => {
                format!("{}!{}{}", my_sheet, str_column, str_row)
            }
            None => {
                format!("{}{}", str_column, str_row)
            }
        }
    }

    pub fn anchor(&mut self, axis: AnchorStyle) {
        match axis {
            AnchorStyle::Row => {
                self.fixed_row = true;
                self.fixed_column = false;
            }
            AnchorStyle::Column => {
                self.fixed_column = true;
                self.fixed_row = false;
            }
            AnchorStyle::All => {
                self.fixed_row = true;
                self.fixed_column = true;
            }
            _ => {
                self.fixed_row = false;
                self.fixed_column = false;
            }
        }
    }

    pub fn unanchor(&mut self) {
        self.fixed_column = false;
        self.fixed_row = false;
    }

    pub fn reposition(&mut self, vertical_offset: i32, horizontal_offset: i32) {
        self.v_reposition(vertical_offset)
            .expect("[wxls] vertical repositioning failed");
        self.h_reposition(horizontal_offset)
            .expect("[wxls] horizontal repositioning failed");
    }

    fn v_reposition(&mut self, vertical_offset: i32) -> Result<(), String> {
        match vertical_offset {
            y if y >= 0 => {
                self.row += y as u32;
                Ok(())
            }
            y => {
                let abs_vert = y.unsigned_abs();
                if self.row >= abs_vert {
                    self.row -= abs_vert;
                    Ok(())
                } else {
                    let msg = format!(
                        "[wxls] repositioning out of bound vertically. \
                        Current row pos is {}. \
                        Attempted re pos is {}",
                        self.row, vertical_offset
                    );
                    console_log!("{}", msg);
                    Err(msg)
                }
            }
        }
    }

    fn h_reposition(&mut self, horizontal_offset: i32) -> Result<(), String> {
        match horizontal_offset {
            x if x >= 0 => {
                self.column += x as u32;
                Ok(())
            }
            x => {
                let abs_hori = x.unsigned_abs();
                if self.column >= abs_hori {
                    self.column -= abs_hori;
                    Ok(())
                } else {
                    let msg = format!(
                        "[wxls] repositioning out of bound horizontally. \
                        Current col pos is {}. \
                        Attempted re pos is {}",
                        self.column, horizontal_offset
                    );
                    console_log!("{}", msg);
                    Err(msg)
                }
            }
        }
    }
}

fn rc_fix(value: &str, fix: bool) -> String {
    if fix {
        let fixed = format!("${}", value);
        fixed
    } else {
        String::from(value)
    }
}
