use crate::cell::*;
use crate::range::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_new() {
    let start_cell = Cell::new(0, 0, None).unwrap();
    let end_cell = Cell::new(1, 1, None).unwrap();

    let range = Range::new(&start_cell, &end_cell).unwrap();

    assert_eq!(range.row_start, start_cell.row);
    assert_eq!(range.row_end, end_cell.row);
    assert_eq!(range.column_start, start_cell.column);
    assert_eq!(range.column_end, end_cell.column);

    assert_eq!(range.to_str_address(), "A1:B2");
}

#[wasm_bindgen_test]
fn test_has() {
    // A1:B2
    let start_cell = Cell::new(0, 0, None).unwrap();
    let end_cell = Cell::new(1, 1, None).unwrap();

    let range = Range::new(&start_cell, &end_cell).unwrap();

    let cell_test1 = Cell::from_str_address("A2", None).unwrap();
    let cell_test2 = Cell::from_str_address("B1", None).unwrap();

    let cell_test3 = Cell::from_str_address("C1", None).unwrap();

    assert!(range.has(&start_cell));
    assert!(range.has(&end_cell));
    assert!(range.has(&cell_test1));
    assert!(range.has(&cell_test2));
    assert!(!range.has(&cell_test3)); // C1 is not included
}

#[wasm_bindgen_test]
fn test_new_include() {
    // A1:B2
    let start_cell = Cell::new(0, 0, None).unwrap();
    let end_cell = Cell::new(1, 1, None).unwrap();

    let range = Range::new(&start_cell, &end_cell).unwrap();

    let cell_test1 = Cell::from_str_address("C1", None).unwrap();

    let new_range = range.new_include(&cell_test1).unwrap();

    assert_eq!(new_range.to_str_address(), "A1:C2");

    // A1:C3
    let start_cell2 = Cell::new(0, 0, None).unwrap();
    let end_cell2 = Cell::from_str_address("C3", None).unwrap();
    let range2 = Range::new(&start_cell2, &end_cell2).unwrap();

    let cell_test2 = Cell::from_str_address("D4", None).unwrap();

    let new_range2 = range2.new_include(&cell_test2).unwrap();

    assert_eq!(new_range2.to_str_address(), "A1:D4");
}
