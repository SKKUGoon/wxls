use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct AddressRC {
    pub start_row: u32,
    pub start_col: u32,

    pub end_row: Option<u32>,
    pub end_col: Option<u32>,

    pub fix_row: bool,
    pub fix_column: bool,
}

impl AddressRC {
    pub fn new(data: Vec<u32>) -> Result<Self, String> {
        // start_anchor and end_anchor
        match data.len() {
            2 => Ok(AddressRC {
                start_row: data[0],
                end_row: None,
                start_col: data[1],
                end_col: None,
                ..Default::default()
            }),
            4 => Ok(AddressRC {
                start_row: data[0],
                end_row: Some(data[1]),
                start_col: data[2],
                end_col: Some(data[3]),
                ..Default::default()
            }),
            _ => Err("Invalid data length".to_string()),
        }
    }

    pub fn from_str_address(data: &str) -> Result<Self, String> {
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
                    Ok(AddressRC {
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
                    Ok(AddressRC {
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

    pub fn to_cell_address(&self) -> String {
        let start_address = rc_to_str(
            &self.start_row,
            &self.start_col,
            self.fix_row,
            self.fix_column,
        );

        match (self.end_row, self.end_col) {
            (Some(r), Some(c)) => {
                let end_address = rc_to_str(&r, &c, self.fix_row, self.fix_column);
                format!("{}:{}", start_address, end_address)
            }
            _ => start_address,
        }
    }

    pub fn anchor_cell_address(&mut self, axis: u8) {
        match axis {
            0u8 => self.fix_row(),
            1u8 => self.fix_col(),
            2u8 => {
                self.fix_row();
                self.fix_col();
            }
            _ => eprintln!("Wrong axis. Nothing happens"),
        }
    }

    fn fix_row(&mut self) {
        self.fix_row = true;
    }

    fn fix_col(&mut self) {
        self.fix_column = true;
    }

    pub fn relocate(&mut self, vertical: u32, horizontal: u32) {
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

impl PartialEq for AddressRC {
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
