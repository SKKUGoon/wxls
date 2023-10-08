use crate::error;
use crate::util::cell_handle::*;
use matches::assert_matches;

#[test]
fn test_r1c1_to_address() {
    assert_eq!(r1c1_to_address(0, 0).unwrap(), "A1");
    assert_eq!(r1c1_to_address(0, 1).unwrap(), "B1");
    assert_eq!(r1c1_to_address(0, 2).unwrap(), "C1");
    assert_eq!(r1c1_to_address(0, 3).unwrap(), "D1");
    assert_eq!(r1c1_to_address(0, 4).unwrap(), "E1");

    assert_eq!(r1c1_to_address(1, 0).unwrap(), "A2");
    assert_eq!(r1c1_to_address(1, 1).unwrap(), "B2");
    assert_eq!(r1c1_to_address(1, 2).unwrap(), "C2");
    assert_eq!(r1c1_to_address(1, 3).unwrap(), "D2");
    assert_eq!(r1c1_to_address(1, 4).unwrap(), "E2");

    assert_eq!(r1c1_to_address(0, 26).unwrap(), "AA1");
    assert_eq!(r1c1_to_address(1, 26).unwrap(), "AA2");
    assert_eq!(r1c1_to_address(2, 26).unwrap(), "AA3");
    assert_eq!(r1c1_to_address(3, 26).unwrap(), "AA4");
    assert_eq!(r1c1_to_address(4, 26).unwrap(), "AA5");

    assert_eq!(r1c1_to_address(0, 27).unwrap(), "AB1");
    assert_eq!(r1c1_to_address(1, 27).unwrap(), "AB2");
    assert_eq!(r1c1_to_address(2, 27).unwrap(), "AB3");
    assert_eq!(r1c1_to_address(3, 27).unwrap(), "AB4");

    assert_eq!(r1c1_to_address(4, 28).unwrap(), "AC5");

    // Excel's last column is XFD. In single worksheet there are 16384 columns
    assert_eq!(r1c1_to_address(1, 16383).unwrap(), "XFD2");
}

#[test]
fn test_r1c1_to_address_oob() {
    assert_matches!(
        r1c1_to_address(1050000, 1),
        Err(error::WebExcelError::OutOfBoundError)
    );
    assert_matches!(
        r1c1_to_address(2000000, 1),
        Err(error::WebExcelError::OutOfBoundError)
    );
    assert_matches!(
        r1c1_to_address(1, 16384),
        Err(error::WebExcelError::OutOfBoundError)
    );
}
