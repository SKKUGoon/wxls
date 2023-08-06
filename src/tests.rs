use super::*;

#[test]
fn test_new() {
    // Test case: Valid input with length 2
    let data1 = vec![1, 2];
    assert_eq!(
        AddressRC::new(data1),
        Ok(AddressRC {
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
        AddressRC::new(data2),
        Ok(AddressRC {
            start_row: 1,
            end_row: Some(2),
            start_col: 3,
            end_col: Some(4),
            ..Default::default()
        })
    );

    // Test case: Invalid input with length other than 2 or 4
    let data3 = vec![1];
    assert_eq!(
        AddressRC::new(data3),
        Err("Invalid data length".to_string())
    );
}

#[test]
fn test_to_cell_address() {
    // Test case: Valid input with length 2
    let data1 = vec![1, 2];
    let addr1 = AddressRC::new(data1).unwrap();
    assert_eq!(addr1.to_cell_address(), String::from("C2"));

    // Test case: Valid input with length 4
    let data2 = vec![1, 2, 3, 4];
    let addr2 = AddressRC::new(data2).unwrap();
    assert_eq!(addr2.to_cell_address(), String::from("D2:E3"));
}

#[test]
fn test_relocate() {
    // Test case: Valid input with length 2
    let data1 = vec![1, 2];
    let mut addr1 = AddressRC::new(data1).unwrap();

    let (move_row1, move_col1) = (2u32, 5u32);
    addr1.relocate(move_row1, move_col1);

    let moved_addr1 = AddressRC::new(vec![3, 7]).unwrap();
    assert_eq!(addr1, moved_addr1);

    // Test case: Valid input with length 4
    let data2 = vec![1, 2, 3, 4];
    let mut addr2 = AddressRC::new(data2).unwrap();

    let (move_row2, move_col2) = (2u32, 5u32);
    addr2.relocate(move_row2, move_col2);

    let moved_addr2 = AddressRC::new(vec![3, 4, 8, 9]).unwrap();
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
