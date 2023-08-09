use crate::{AddressComponent, Cell};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Range {
    sheet: &'static str,
    address: Cell,
}

// #[wasm_bindgen]
impl Range {
    pub fn new(sheet: &'static str, rc_address: Cell) -> Self {
        Range {
            sheet,
            address: rc_address,
        }
    }

    pub fn to_str_address(&self) -> String {
        let front = format!(
            "{}!{}",
            self.sheet,
            self.address
                .address_component(AddressComponent::Front)
                .unwrap()
        );

        if let Ok(back) = self.address.address_component(AddressComponent::Back) {
            format!("{}:{}!{}", front, self.sheet, back)
        } else {
            front
        }
    }

    pub fn has(&self, other: Range) -> bool {
        if other.sheet != self.sheet {
            return false;
        }

        if other.address == self.address {
            return true;
        }

        let start_row_inside = self.address.start_row <= other.address.start_row;
        let start_col_inside = self.address.start_col <= other.address.start_col;
        let end_row_inside = match (self.address.end_row, other.address.end_row) {
            (Some(ser), Some(oer)) => ser >= oer,
            (Some(_), None) => true,
            _ => false,
        };
        let end_col_inside = match (self.address.end_col, other.address.end_col) {
            (Some(sec), Some(oec)) => sec >= oec,
            (Some(_), None) => true,
            _ => false,
        };

        start_row_inside && start_col_inside && end_row_inside && end_col_inside
    }

    pub fn new_include(self, other: Range) -> Result<Range, String> {
        if other.sheet != self.sheet {
            return Err("Cannot include other sheet".to_string());
        }

        if self.has(other) {
            return Ok(self);
        }

        // Change address to include `other`
        let new_start_row = std::cmp::min(self.address.start_row, other.address.start_col);
        let new_start_col = std::cmp::min(self.address.start_col, other.address.start_col);

        let new_end_row = match (self.address.end_row, other.address.end_row) {
            (Some(ser), Some(oer)) => std::cmp::max(ser, oer),
            (Some(ser), None) => std::cmp::max(ser, other.address.start_row),
            (None, Some(oer)) => std::cmp::max(self.address.start_row, oer),
            _ => std::cmp::max(self.address.start_row, other.address.start_row),
        };

        let new_end_col = match (self.address.end_col, other.address.end_col) {
            (Some(sec), Some(oec)) => std::cmp::max(sec, oec),
            (Some(sec), None) => std::cmp::max(sec, other.address.start_col),
            (None, Some(oec)) => std::cmp::max(self.address.start_col, oec),
            _ => std::cmp::max(self.address.start_col, other.address.start_col),
        };

        let new_cell =
            Cell::new(vec![new_start_row, new_end_row, new_start_col, new_end_col]).unwrap();

        Ok(Range {
            sheet: self.sheet,
            address: new_cell,
        })
    }

    pub fn force_include(&mut self, other: Range) -> Result<&Range, String> {
        if other.sheet != self.sheet {
            return Err("Cannot include other sheet".to_string());
        }

        if self.has(other) {
            return Ok(self);
        }

        // Change address to include `other`
        self.address.start_row = std::cmp::min(self.address.start_row, other.address.start_col);
        self.address.start_col = std::cmp::min(self.address.start_col, other.address.start_col);

        self.address.end_row = match (self.address.end_row, other.address.end_row) {
            (Some(ser), Some(oer)) => Some(std::cmp::max(ser, oer)),
            (Some(ser), None) => Some(std::cmp::max(ser, other.address.start_row)),
            (None, Some(oer)) => Some(std::cmp::max(self.address.start_row, oer)),
            _ => Some(std::cmp::max(
                self.address.start_row,
                other.address.start_row,
            )),
        };

        self.address.end_col = match (self.address.end_col, other.address.end_col) {
            (Some(sec), Some(oec)) => Some(std::cmp::max(sec, oec)),
            (Some(sec), None) => Some(std::cmp::max(sec, other.address.start_col)),
            (None, Some(oec)) => Some(std::cmp::max(self.address.start_col, oec)),
            _ => Some(std::cmp::max(
                self.address.start_col,
                other.address.start_col,
            )),
        };

        Ok(self)
    }

    // pub fn envelope<T>(matrix: Vec<Vec<T>>) {

    // }
}
