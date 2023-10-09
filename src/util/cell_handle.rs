use crate::{cell, error};

/// Converts an Excel-like cell address (e.g., "A1", "BC23") to its corresponding row and column indices.
///
/// The function expects the address to be in the format of one or more uppercase letters followed by one or more digits.
/// The letters represent the column (e.g., "A" for the first column, "Z" for the 26th column, "AA" for the 27th column, etc.),
/// and the digits represent the row (1-based index).
///
/// # Arguments
///
/// * `addr` - A string slice containing the Excel-like cell address.
///
/// # Returns
///
/// * `Ok(cell::Cell)` - A `Cell` struct containing the 0-based row and column indices.
/// * `Err(error::WebExcelError::ParseError)` - An error indicating that the input address is not in the expected format or contains invalid characters.
///
/// # Examples
///
/// ```
/// let cell = address_to_r1c1("A1").unwrap();
/// assert_eq!(cell.row, 0);
/// assert_eq!(cell.column, 0);
///
/// let cell = address_to_r1c1("Z26").unwrap();
/// assert_eq!(cell.row, 25);
/// assert_eq!(cell.column, 25);
/// ```
///
pub fn address_to_r1c1(addr: &str) -> Result<cell::Cell, error::WebExcelError> {
    let chars = addr.chars();
    let mut column = 0isize;
    let mut row = 0isize;

    // Cell address contains 2 parts:
    // Row in positive integer(capture with is_ascii_digit)
    // Column in capitalized alphabet(capture with is_ascii_alphabetic)
    for c in chars {
        if c.is_ascii_digit() {
            row = row * 10 + (c as usize - '0' as usize) as isize;
        } else if c.is_ascii_alphabetic() {
            column = column * 26 + (c.to_ascii_uppercase() as usize - 'A' as usize) as isize + 1;
        } else {
            return Err(error::WebExcelError::ParseError);
        }
    }

    if row - 1 < 0 || column - 1 < 0 {
        return Err(error::WebExcelError::ParseError);
    }

    Ok(cell::Cell {
        row: (row - 1) as u32,
        column: (column - 1) as u32,
        ..Default::default()
    })
}

/// Converts zero index based row, column indices to its corresponding Excel-like cell address.
/// Opposite of function `address_to_r1c1`.
///
/// The function expects to take zero-starting index.
/// For example, cell "A2" has "A" for column part and "2" for row part.
/// "A" is the first column so it would be indexed '0'. "2" is the second row, so it would be indexed "1".
///
/// # Arguments
///
/// * `row` - row index (starting from 0)
/// * `col` - column index (starting from 0)
///
/// # Returns
///
/// * `String` - An owned String address that represents the address on Excel worksheet.
/// * `Err(error::WebExcelError::ParseError)` - An error indicating that the input address is not in the expected format or contains invalid characters.
///
/// # Examples
///
/// ```
/// assert_eq!(r1c1_to_address(3, 27), "AB4");
/// assert_eq!(r1c1_to_address(4, 28), "AC5");
/// assert_eq!(r1c1_to_address(1, 16383), "XFD2");
/// ```
///
pub fn r1c1_to_address(
    row: u32,
    col: u32,
    row_lock: bool,
    col_lock: bool,
) -> Result<String, error::WebExcelError> {
    // Input check
    if col > 16383 || row > 1048575 {
        return Err(error::WebExcelError::OutOfBoundError);
    }

    // Row
    let addr_row = row + 1;

    // Column
    let mut addr_col = col + 1;
    let mut elements: Vec<char> = vec![];

    if row_lock {
        elements.push('$');
    }

    if addr_col == 0 {
        elements.push('A');
    }

    while addr_col > 0 {
        addr_col -= 1;
        let remainder = (addr_col % 26) as u8;
        let pos = (remainder + b'A') as char;
        elements.push(pos);
        addr_col /= 26;
    }

    if col_lock {
        elements.push('$');
    }

    elements.reverse();
    Ok(format!(
        "{}{}",
        elements.iter().collect::<String>(),
        addr_row
    ))
}
