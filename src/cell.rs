use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct Cell {
    pub start_row: u32,
    pub start_col: u32,

    pub end_row: Option<u32>,
    pub end_col: Option<u32>,

    pub fix_row: bool,
    pub fix_column: bool,

    pub fix_start: bool,
    pub fix_end: bool,
}

/// axis 0: A1 => A$1
/// axis 1: A1 => $A1
/// axis 2: A1 => $A$1
#[wasm_bindgen]
pub enum AnchorStyle {
    Row = 0,
    Column = 1,
    All = 2,
}

/// cell 0: A1:B1 => only A1 is affected
/// cell 1: A1:B1 => only B1 is affected
/// cell 2: A1:B1 => Both A1, B1 is affected
#[wasm_bindgen]
pub enum AddressComponent {
    Front = 0,
    Back = 1,
    Both = 2,
}

#[wasm_bindgen]
impl Cell {
    pub fn new(data: Vec<u32>) -> Result<Cell, String> {
        // start_anchor and end_anchor
        match data.len() {
            2 => Ok(Cell {
                start_row: data[0],
                end_row: None,
                start_col: data[1],
                end_col: None,
                ..Default::default()
            }),
            4 => Ok(Cell {
                start_row: data[0],
                start_col: data[2],

                end_row: if data[0] != data[1] {
                    Some(data[1])
                } else {
                    None
                },
                end_col: if data[2] != data[3] {
                    Some(data[3])
                } else {
                    None
                },
                ..Default::default()
            }),
            _ => Err("Invalid data length".to_string()),
        }
    }

    pub fn from_str_address(data: &str) -> Result<Cell, String> {
        match data.contains(':') {
            true => {
                let cells: Vec<&str> = data.split(':').collect();
                if cells.len() != 2 {
                    return Err("Invalid origin string".to_string());
                }

                let start_cell = cells[0];
                let end_cell = cells[1];

                if let (Some((sr, sc)), Some((er, ec))) =
                    (str_to_rc(start_cell), str_to_rc(end_cell))
                {
                    Ok(Cell {
                        start_row: sr,
                        start_col: sc,
                        end_row: Some(er),
                        end_col: Some(ec),
                        ..Default::default()
                    })
                } else {
                    Err("Invalid origin string".to_string())
                }
            }
            false => {
                if let Some((r, c)) = str_to_rc(data) {
                    Ok(Cell {
                        start_row: r,
                        start_col: c,
                        end_row: None,
                        end_col: None,
                        ..Default::default()
                    })
                } else {
                    Err("Invalid origin string".to_string())
                }
            }
        }
    }

    pub fn to_str_address(&self) -> String {
        let start_address = rc_to_str(
            &self.start_row,
            &self.start_col,
            self.fix_row && self.fix_start,
            self.fix_column && self.fix_start,
        );

        match (self.end_row, self.end_col) {
            (Some(r), Some(c)) => {
                let end_address = rc_to_str(
                    &r,
                    &c,
                    self.fix_row && self.fix_end,
                    self.fix_column && self.fix_end,
                );
                format!("{}:{}", start_address, end_address)
            }
            _ => start_address,
        }
    }

    pub fn address_component(&self, cell: AddressComponent) -> Result<String, String> {
        match cell {
            AddressComponent::Front => Ok(rc_to_str(
                &self.start_row,
                &self.start_col,
                self.fix_row && self.fix_start,
                self.fix_column && self.fix_start,
            )),
            AddressComponent::Back => match (self.end_row, self.end_col) {
                (Some(r), Some(c)) => Ok(rc_to_str(
                    &r,
                    &c,
                    self.fix_row && self.fix_end,
                    self.fix_column && self.fix_end,
                )),
                _ => Err(String::from("no second component")),
            },
            _ => Err(String::from("Should choose only one")),
        }
    }

    pub fn anchor(&mut self, axis: AnchorStyle, cell: AddressComponent) {
        match axis {
            AnchorStyle::Row => {
                self.fix_row(&cell);
                self.fix_column = false;
            }
            AnchorStyle::Column => {
                self.fix_col(&cell);
                self.fix_row = false;
            }
            AnchorStyle::All => {
                self.fix_row(&cell);
                self.fix_col(&cell);
            }
        }
    }

    pub fn unanchor(&mut self) {
        self.fix_column = false;
        self.fix_row = false;
        self.fix_start = false;
        self.fix_end = false;
    }

    fn fix_row(&mut self, cell: &AddressComponent) {
        self.fix_row = true;
        match cell {
            AddressComponent::Front => {
                self.fix_start = true;
                self.fix_end = false;
            }
            AddressComponent::Back => {
                self.fix_start = false;
                self.fix_end = true;
            }
            AddressComponent::Both => {
                self.fix_start = true;
                self.fix_end = true
            }
        }
    }

    fn fix_col(&mut self, cell: &AddressComponent) {
        self.fix_column = true;
        match cell {
            AddressComponent::Front => {
                self.fix_start = true;
                self.fix_end = false;
            }
            AddressComponent::Back => {
                self.fix_start = false;
                self.fix_end = true;
            }
            AddressComponent::Both => {
                self.fix_start = true;
                self.fix_end = true
            }
        }
    }

    pub fn movement(&mut self, vertical: u32, horizontal: u32) {
        // Move row
        if let Some(end_row) = self.end_row {
            self.end_row = Some(end_row + vertical);
        }

        self.start_row += vertical;

        // Move column
        if let Some(end_col) = self.end_col {
            self.end_col = Some(end_col + horizontal);
        }

        self.start_col += horizontal;
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.start_row == other.start_row
            && self.end_row == other.end_row
            && self.start_col == other.start_col
            && self.end_col == other.end_col
    }
}

pub fn rc_to_str(row: &u32, col: &u32, rowfix: bool, colfix: bool) -> String {
    let wr = row + 1;
    let mut wc = col + 1;

    let mut str_column = String::from("");
    str_column = loop {
        let remain = (wc - 1) % 26;
        let name_element = char::from_u32(65u32 + remain).unwrap();

        str_column.push(name_element);
        wc = (wc - remain) / 26;
        if wc == 0 {
            break rc_fix(&str_column, colfix);
        }
    };

    let str_row = rc_fix(&wr.to_string(), rowfix);

    format!("{}{}", str_column, str_row)
}

fn rc_fix(value: &str, fix: bool) -> String {
    if fix {
        let fixed = format!("${}", value);
        fixed
    } else {
        String::from(value)
    }
}

pub fn str_to_rc(address: &str) -> Option<(u32, u32)> {
    let chars = address.chars();
    let mut column = 0isize;
    let mut row = 0isize;

    for c in chars {
        if c.is_ascii_digit() {
            row = row * 10 + (c as usize - '0' as usize) as isize;
        } else if c.is_ascii_alphabetic() {
            column = column * 26 + (c.to_ascii_uppercase() as usize - 'A' as usize) as isize + 1;
        } else {
            return None;
        }
    }

    if row - 1 < 0 || column - 1 < 0 {
        return None;
    }

    Some(((row - 1) as u32, (column - 1) as u32))
}

pub fn str_is_fix(address: &str) -> (bool, bool) {
    let mut chars = address.chars().peekable();
    let mut row_fixed = false;
    let mut col_fixed = false;

    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some(next_char) = chars.peek() {
                if next_char.is_ascii_alphabetic() {
                    col_fixed = true;
                } else if next_char.is_ascii_digit() {
                    row_fixed = true;
                }
            }
        }
    }

    (row_fixed, col_fixed)
}
