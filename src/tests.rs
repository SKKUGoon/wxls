use super::*;

#[test]
fn test_new() {
    // Test case: Valid input with length 2
    let data1 = vec![1, 2];
    assert_eq!(
        Cell::new(data1),
        Ok(Cell {
            start_row: 1,
            end_row: None,
            start_col: 2,
            end_col: None,
            ..Default::default()
        })
    );

    // Test case: Valid input with length 4
    let data2 = vec![1, 2, 3, 4];
    assert_eq!(
        Cell::new(data2),
        Ok(Cell {
            start_row: 1,
            end_row: Some(2),
            start_col: 3,
            end_col: Some(4),
            ..Default::default()
        })
    );

    // Test case: Invalid input with length other than 2 or 4
    let data3 = vec![1];
    assert_eq!(Cell::new(data3), Err("Invalid data length".to_string()));
}

#[test]
fn test_from_str() {
    let single_cell = "B1";
    let from_string = Cell::from_str_address(single_cell);
    let from_index = Cell::new(vec![0, 1]);
    assert_eq!(from_index, from_string);

    let multi_cell = "A9:D12";
    let from_string = Cell::from_str_address(multi_cell);
    let from_index = Cell::new(vec![8, 11, 0, 3]);
    assert_eq!(from_index, from_string);
}

#[test]
fn test_to_cell_address() {
    // Test case: Valid input with length 2
    let data1 = vec![1, 2];
    let addr1 = Cell::new(data1).unwrap();
    assert_eq!(addr1.to_str_address(), String::from("C2"));

    // Test case: Valid input with length 4
    let data2 = vec![1, 2, 3, 4];
    let addr2 = Cell::new(data2).unwrap();
    assert_eq!(addr2.to_str_address(), String::from("D2:E3"));
}

#[test]
fn test_to_cell_address_fix() {
    // Test case: Valid input with length 2 and fixed
    let data1 = vec![1, 2];
    let mut addr1 = Cell::new(data1).unwrap();
    addr1.anchor(0, 0);
    assert_eq!(addr1.to_str_address(), String::from("C$2"));

    addr1.anchor(1, 0);
    assert_eq!(addr1.to_str_address(), String::from("$C2"));

    addr1.anchor(2, 0);
    assert_eq!(addr1.to_str_address(), String::from("$C$2"));

    let data2 = vec![1, 2, 3, 4];
    let mut addr2 = Cell::new(data2).unwrap();
    addr2.anchor(0, 0);
    assert_eq!(addr2.to_str_address(), String::from("D$2:E3"));

    addr2.anchor(0, 1);
    assert_eq!(addr2.to_str_address(), String::from("D2:E$3"));

    addr2.anchor(0, 2);
    assert_eq!(addr2.to_str_address(), String::from("D$2:E$3"));

    addr2.anchor(1, 0);
    assert_eq!(addr2.to_str_address(), String::from("$D2:E3"));

    addr2.anchor(1, 1);
    assert_eq!(addr2.to_str_address(), String::from("D2:$E3"));

    addr2.anchor(1, 2);
    assert_eq!(addr2.to_str_address(), String::from("$D2:$E3"));

    addr2.anchor(2, 0);
    assert_eq!(addr2.to_str_address(), String::from("$D$2:E3"));

    addr2.anchor(2, 1);
    assert_eq!(addr2.to_str_address(), String::from("D2:$E$3"));

    addr2.anchor(2, 2);
    assert_eq!(addr2.to_str_address(), String::from("$D$2:$E$3"));
}

#[test]
fn test_relocate() {
    // Test case: Valid input with length 2
    let data1 = vec![1, 2];
    let mut addr1 = Cell::new(data1).unwrap();

    let (move_row1, move_col1) = (2u32, 5u32);
    addr1.movement(move_row1, move_col1);

    let moved_addr1 = Cell::new(vec![3, 7]).unwrap();
    assert_eq!(addr1, moved_addr1);

    // Test case: Valid input with length 4
    let data2 = vec![1, 2, 3, 4];
    let mut addr2 = Cell::new(data2).unwrap();

    let (move_row2, move_col2) = (2u32, 5u32);
    addr2.movement(move_row2, move_col2);

    let moved_addr2 = Cell::new(vec![3, 4, 8, 9]).unwrap();
    assert_eq!(addr2, moved_addr2);
}

#[test]
fn test_rc_to_str() {
    let (row, col) = (3u32, 5u32);
    let addr1 = rc_to_str(&row, &col, false, false);
    assert_eq!(addr1, "F4");

    let addr2 = rc_to_str(&row, &col, true, false);
    assert_eq!(addr2, "F$4");

    let addr3 = rc_to_str(&row, &col, false, true);
    assert_eq!(addr3, "$F4");

    let addr4 = rc_to_str(&row, &col, true, true);
    assert_eq!(addr4, "$F$4");
}

#[test]
fn test_str_to_rc() {
    let cell_addr1 = "A1";
    let rc_addr1 = str_to_rc(cell_addr1).unwrap();
    assert_eq!(rc_addr1, (0, 0));

    let cell_addr2 = "C4";
    let rc_addr2 = str_to_rc(cell_addr2).unwrap();
    assert_eq!(rc_addr2, (3, 2));
}

#[test]
fn test_str_is_fix() {
    let cell_addr1 = "A$1";
    let rc_addr1 = str_is_fix(cell_addr1);
    assert_eq!(rc_addr1, (true, false));

    let cell_addr2 = "C4";
    let rc_addr2 = str_is_fix(cell_addr2);
    assert_eq!(rc_addr2, (false, false));

    let cell_addr3 = "$CC4";
    let rc_addr2 = str_is_fix(cell_addr3);
    assert_eq!(rc_addr2, (false, true));

    let cell_addr4 = "$XFC$4D";
    let rc_addr2 = str_is_fix(cell_addr4);
    assert_eq!(rc_addr2, (true, true));
}

#[test]
fn test_to_cell() {
    let data1 = vec![1, 2];
    let addr = Cell::new(data1).unwrap();
    let cell = Range::new("Sheet1", addr);
    assert_eq!(cell.to_str_address(), String::from("Sheet1!C2"));
}
