use dynstr::{DynamicString, IndexedString};

#[test]
fn test_basic() {
    let str = DynamicString::new("0123456789");
    let indexed = IndexedString::new(str);
    assert_eq!(indexed.at(0), '0' as u16);
    assert_eq!(indexed.at(5), '5' as u16);
    assert_eq!(indexed.len(), 10);
}

#[test]
fn test_cons() {
    let str = DynamicString::ConsString {
        first: Box::new(DynamicString::new("012345")),
        second: Box::new(DynamicString::new("6789")),
    };
    let indexed = IndexedString::new(str);
    assert_eq!(indexed.at(2), '2' as u16);
    assert_eq!(indexed.at(5), '5' as u16);
    assert_eq!(indexed.at(6), '6' as u16);
    assert_eq!(indexed.at(7), '7' as u16);
    assert_eq!(indexed.at(9), '9' as u16);
    assert_eq!(indexed.len(), 10);
}

#[test]
fn test_slice() {
    let str = DynamicString::SlicedString {
        root: Box::new(DynamicString::new("aa0123456789")),
        start: 2,
        length: 5,
    };
    let indexed = IndexedString::new(str);
    assert_eq!(indexed.at(2), '2' as u16);
    assert_eq!(indexed.at(4), '4' as u16);
    assert_eq!(indexed.len(), 5);
}

#[test]
#[should_panic]
fn test_slice_panic() {
    let str = DynamicString::SlicedString {
        root: Box::new(DynamicString::new("aa0123456789")),
        start: 2,
        length: 5,
    };
    let indexed = IndexedString::new(str);
    assert_eq!(indexed.at(5), '5' as u16);
}

#[test]
fn test_slice_cons() {
    let sliced = DynamicString::SlicedString {
        root: Box::new(DynamicString::new("aa0123456789")),
        start: 2,
        length: 5,
    };
    // 01234-01234
    let str = DynamicString::ConsString {
        first: Box::new(sliced.clone()),
        second: Box::new(DynamicString::ConsString {
            first: Box::new(DynamicString::new("-")),
            second: Box::new(sliced.clone()),
        }),
    };
    let indexed = IndexedString::new(str);
    assert_eq!(indexed.at(0), '0' as u16);
    assert_eq!(indexed.at(4), '4' as u16);
    assert_eq!(indexed.at(5), '-' as u16);
    assert_eq!(indexed.at(6), '0' as u16);
    assert_eq!(indexed.at(7), '1' as u16);
    assert_eq!(indexed.at(10), '4' as u16);
    assert_eq!(indexed.len(), 11);
}

#[test]
fn test_slice_cons_slice() {
    let sliced = DynamicString::SlicedString {
        root: Box::new(DynamicString::new("aa0123456789")),
        start: 2,
        length: 5,
    };
    let root = DynamicString::ConsString {
        first: Box::new(sliced.clone()),
        second: Box::new(DynamicString::ConsString {
            first: Box::new(DynamicString::new("-")),
            second: Box::new(sliced.clone()),
        }),
    };
    let str = DynamicString::SlicedString {
        root: Box::new(root),
        start: 4,
        length: 3,
    };
    let indexed = IndexedString::new(str);
    assert_eq!(indexed.at(0), '4' as u16);
    assert_eq!(indexed.at(1), '-' as u16);
    assert_eq!(indexed.at(2), '0' as u16);
    assert_eq!(indexed.len(), 3);
}
