use crate::error;
use crate::util::cell_handle::*;
use matches::assert_matches;

#[test]
fn test_r1c1_to_address() {
    assert_eq!(r1c1_to_address(0, 0, false, false).unwrap(), "A1");
    assert_eq!(r1c1_to_address(0, 1, true, false).unwrap(), "B$1");
    assert_eq!(r1c1_to_address(0, 2, false, true).unwrap(), "$C1");
    assert_eq!(r1c1_to_address(0, 3, false, false).unwrap(), "D1");
    assert_eq!(r1c1_to_address(0, 4, false, false).unwrap(), "E1");

    assert_eq!(r1c1_to_address(1, 0, false, false).unwrap(), "A2");
    assert_eq!(r1c1_to_address(1, 1, false, false).unwrap(), "B2");
    assert_eq!(r1c1_to_address(1, 2, false, false).unwrap(), "C2");
    assert_eq!(r1c1_to_address(1, 3, false, false).unwrap(), "D2");
    assert_eq!(r1c1_to_address(1, 4, false, false).unwrap(), "E2");

    assert_eq!(r1c1_to_address(0, 26, false, false).unwrap(), "AA1");
    assert_eq!(r1c1_to_address(1, 26, false, false).unwrap(), "AA2");
    assert_eq!(r1c1_to_address(2, 26, false, false).unwrap(), "AA3");
    assert_eq!(r1c1_to_address(3, 26, false, false).unwrap(), "AA4");
    assert_eq!(r1c1_to_address(4, 26, false, false).unwrap(), "AA5");

    assert_eq!(r1c1_to_address(0, 27, false, false).unwrap(), "AB1");
    assert_eq!(r1c1_to_address(1, 27, false, false).unwrap(), "AB2");
    assert_eq!(r1c1_to_address(2, 27, false, false).unwrap(), "AB3");
    assert_eq!(r1c1_to_address(3, 27, false, false).unwrap(), "AB4");

    assert_eq!(r1c1_to_address(4, 28, false, false).unwrap(), "AC5");

    // Excel's last column is XFD. In single worksheet there are 16384 columns
    assert_eq!(r1c1_to_address(1, 16383, false, false).unwrap(), "XFD2");
}

#[test]
fn test_r1c1_to_address_oob() {
    assert_matches!(
        r1c1_to_address(1050000, 1, false, false),
        Err(error::WebExcelError::OutOfBoundError)
    );
    assert_matches!(
        r1c1_to_address(2000000, 1, false, false),
        Err(error::WebExcelError::OutOfBoundError)
    );
    assert_matches!(
        r1c1_to_address(1, 16384, false, false),
        Err(error::WebExcelError::OutOfBoundError)
    );
}
