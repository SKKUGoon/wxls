use crate::cell::*;
use crate::range::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_range_new() {
    // Define base cells
    let cell_one = Cell::from_str_address("A1", None).unwrap();
    let cell_two = Cell::from_str_address("B2", None).unwrap();

    let cell_inv_one = Cell::from_str_address("B1", None).unwrap();
    let cell_inv_two = Cell::from_str_address("A2", None).unwrap();

    // Initiate range defined by two cells
    let range_one = Range::new(&cell_one, &cell_two).unwrap();
    let range_two = Range::new(&cell_two, &cell_one).unwrap();
    let range_three = Range::new(&cell_inv_one, &cell_inv_two).unwrap();
    let range_four = Range::new(&cell_inv_two, &cell_inv_one).unwrap();

    // Test equality of range
    assert_eq!(range_one, range_two);
    assert_eq!(range_one, range_three);
    assert_eq!(range_one, range_four);
}

#[wasm_bindgen_test]
fn test_range_to_str_address() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("B2", None).unwrap();

    let cell31 = Cell::from_str_address("B1", None).unwrap();
    let cell32 = Cell::from_str_address("A2", None).unwrap();

    let cell21 = Cell::from_str_address("AA1", None).unwrap();
    let cell22 = Cell::from_str_address("AA2", None).unwrap();

    // Define base cells with sheet name
    let sheet_name = "Sheet1";
    let cell41 = Cell::from_str_address("AB2", Some(sheet_name.to_owned())).unwrap();
    let cell42 = Cell::from_str_address("AB5", Some(sheet_name.to_owned())).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();
    let range3 = Range::new(&cell31, &cell32).unwrap();
    let range2 = Range::new(&cell21, &cell22).unwrap();

    // Initiate range defined by two base cells with sheet name
    let range4 = Range::new(&cell41, &cell42).unwrap();

    assert_eq!(range1.to_str_address().unwrap(), "A1:B2");
    assert_eq!(range3.to_str_address().unwrap(), "A1:B2"); // Check that even though the cell position are skewed, it returns the correct range
    assert_eq!(range2.to_str_address().unwrap(), "AA1:AA2");
    assert_eq!(range4.to_str_address().unwrap(), "Sheet1!AB2:Sheet1!AB5");
}

#[wasm_bindgen_test]
fn test_range_has() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("Z100", None).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();

    // Testing data - has true
    let cell1 = Cell::from_str_address("A1", None).unwrap();
    let cell2 = Cell::from_str_address("B12", None).unwrap();
    let cell3 = Cell::from_str_address("C23", None).unwrap();
    let cell4 = Cell::from_str_address("D34", None).unwrap();
    let cell5 = Cell::from_str_address("Z100", None).unwrap();

    // Testing data - has false
    let cell6 = Cell::from_str_address("AA61", None).unwrap();
    let cell7 = Cell::from_str_address("AA1", None).unwrap();
    let cell8 = Cell::from_str_address("Z110", None).unwrap();
    let cell9 = Cell::from_str_address("XFD123", None).unwrap();
    let cell0 = Cell::from_str_address("DD123", None).unwrap();

    assert!(range1.has(&cell1));
    assert!(range1.has(&cell2));
    assert!(range1.has(&cell3));
    assert!(range1.has(&cell4));
    assert!(range1.has(&cell5));

    assert!(!range1.has(&cell6));
    assert!(!range1.has(&cell7));
    assert!(!range1.has(&cell8));
    assert!(!range1.has(&cell9));
    assert!(!range1.has(&cell0));
}

#[wasm_bindgen_test]
fn test_range_includes() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("Z100", None).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();

    // Testing data - includes true
    let i_cell11 = Cell::from_str_address("A2", None).unwrap();
    let i_cell12 = Cell::from_str_address("Z1", None).unwrap();
    let i_range = Range::new(&i_cell11, &i_cell12).unwrap();

    // Testing data - includes false
    let ni_cell11 = Cell::from_str_address("A2", None).unwrap();
    let ni_cell12 = Cell::from_str_address("Z101", None).unwrap();
    let ni_range = Range::new(&ni_cell11, &ni_cell12).unwrap();

    assert!(range1.includes(&i_range));
    assert!(!range1.includes(&ni_range));
}

#[wasm_bindgen_test]
fn test_range_iter_col() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("Z100", None).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();

    let columns = range1.iter_col().unwrap();

    // console_log!("{:?}", columns);
    println!("{:?}", columns);
}

#[wasm_bindgen_test]
fn test_range_iter_row() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("Z100", None).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();

    let rows = range1.iter_row().unwrap();

    // console_log!("{:?}", rows);
    println!("{:?}", rows);
}

#[wasm_bindgen_test]
fn test_range_select_row() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("Z100", None).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();

    // Select row by row
    let row1 = range1.select_row(0, 1).unwrap();
    assert_eq!(row1.to_str_address().unwrap(), "A1:Z1");

    let row2 = range1.select_row(1, 2).unwrap();
    assert_eq!(row2.to_str_address().unwrap(), "A2:Z2");

    let row3 = range1.select_row(0, 10).unwrap();
    assert_eq!(row3.to_str_address().unwrap(), "A1:Z10");
}

#[wasm_bindgen_test]
fn test_range_select_col() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("Z100", None).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();

    // Select column by column
    let column1 = range1.select_column(0, 1).unwrap();
    assert_eq!(column1.to_str_address().unwrap(), "A1:A100");

    let column2 = range1.select_column(2, 4).unwrap();
    assert_eq!(column2.to_str_address().unwrap(), "C1:D100");

    let column3 = range1.select_column(0, 5).unwrap();
    assert_eq!(column3.to_str_address().unwrap(), "A1:E100");
}

#[wasm_bindgen_test]
fn test_range_intersects() {
    // Define base cells
    let cell11 = Cell::from_str_address("A1", None).unwrap();
    let cell12 = Cell::from_str_address("Z100", None).unwrap();

    let cell21 = Cell::from_str_address("Z98", None).unwrap();
    let cell22 = Cell::from_str_address("AB12", None).unwrap();

    // Initiate range defined by two base cells
    let range1 = Range::new(&cell11, &cell12).unwrap();
    let range2 = Range::new(&cell21, &cell22).unwrap();

    assert!(range1.intersects(&range2).unwrap());
    assert!(range2.intersects(&range1).unwrap());
}
