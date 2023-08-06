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

    pub fn to_cell_address(&self) -> String {
        let start_address = rc_to_str(&self.start_row, &self.start_col);

        match (self.end_row, self.end_col) {
            (Some(r), Some(c)) => {
                let end_address = rc_to_str(&r, &c);
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

pub fn rc_to_str(row: &u32, col: &u32) -> String {
    let wr = row + 1;
    let mut wc = col + 1;

    let mut str_address = String::from("");
    str_address = loop {
        let remain = (wc - 1) % 26;
        let name_element = char::from_u32(65u32 + remain).unwrap();

        str_address.push(name_element);
        wc = (wc - remain) / 26;
        if wc == 0 {
            break str_address;
        }
    };

    str_address.push(std::char::from_digit(wr, 10).unwrap());

    str_address
}
