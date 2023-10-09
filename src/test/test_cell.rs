use crate::cell::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_new() {
    let data: Vec<(u32, u32)> = vec![(0, 0)];
    let result = Cell {
        row: 0,
        column: 0,
        ..Default::default()
    };

    assert_eq!(Cell::new(data[0].0, data[0].1, None).unwrap(), result);
}

#[wasm_bindgen_test]
fn test_from_str_addr() {
    let cell_addrs: Vec<&str> = vec!["B1", "C2", "D5", "X41", "AA11"];
    let from_index: Vec<(u32, u32)> = vec![(0, 1), (1, 2), (4, 3), (40, 23), (10, 26)];

    for (letter, idx) in cell_addrs.into_iter().zip(from_index.into_iter()) {
        let cell_letter = Cell::from_str_address(letter, None);
        let cell_idx = Cell::new(idx.0, idx.1, None);
        assert_eq!(cell_letter.unwrap(), cell_idx.unwrap());
    }
}

#[wasm_bindgen_test]
fn test_to_cell_address() {
    let from_index: Vec<(u32, u32)> = vec![(0, 1), (1, 2), (4, 3), (40, 23), (10, 26)];
    let cell_addrs: Vec<&str> = vec!["B1", "C2", "D5", "X41", "AA11"];

    for (letter, idx) in cell_addrs.into_iter().zip(from_index.into_iter()) {
        let cell_idx = Cell::new(idx.0, idx.1, None).unwrap();
        assert_eq!(cell_idx.to_str_address().unwrap(), letter.to_owned())
    }
}

#[wasm_bindgen_test]
fn test_to_cell_address_fixed() {
    let from_index: Vec<(u32, u32)> = vec![(0, 1), (1, 2), (4, 3), (40, 23), (10, 26)];
    let cell_addrs: Vec<&str> = vec!["$B$1", "$C$2", "$D$5", "$X$41", "$AA$11"];

    for (letter, idx) in cell_addrs.into_iter().zip(from_index.into_iter()) {
        let mut cell_idx = Cell::new(idx.0, idx.1, None).unwrap();
        cell_idx.anchor(AnchorStyle::All);
        assert_eq!(cell_idx.to_str_address().unwrap(), letter.to_owned())
    }
}

#[wasm_bindgen_test]
fn test_to_cell_address_column_fix() {
    let from_index: Vec<(u32, u32)> = vec![(0, 1), (1, 2), (4, 3), (40, 23), (10, 26)];
    let cell_addrs: Vec<&str> = vec!["$B1", "$C2", "$D5", "$X41", "$AA11"];

    for (letter, idx) in cell_addrs.into_iter().zip(from_index.into_iter()) {
        let mut cell_idx = Cell::new(idx.0, idx.1, None).unwrap();
        cell_idx.anchor(AnchorStyle::Column);
        assert_eq!(cell_idx.to_str_address().unwrap(), letter.to_owned())
    }
}

#[wasm_bindgen_test]
fn test_to_cell_address_row_fix() {
    let from_index: Vec<(u32, u32)> = vec![(0, 1), (1, 2), (4, 3), (40, 23), (10, 26)];
    let cell_addrs: Vec<&str> = vec!["B$1", "C$2", "D$5", "X$41", "AA$11"];

    for (letter, idx) in cell_addrs.into_iter().zip(from_index.into_iter()) {
        let mut cell_idx = Cell::new(idx.0, idx.1, None).unwrap();
        cell_idx.anchor(AnchorStyle::Row);
        assert_eq!(cell_idx.to_str_address().unwrap(), letter.to_owned())
    }
}
