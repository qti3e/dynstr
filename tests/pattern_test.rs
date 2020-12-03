use dynstr::{DynamicString, PatternFinder};

#[test]
fn test_basic() {
    let text = DynamicString::new("Hello world, I live in a world.");
    let pattern = DynamicString::new("world");
    assert_eq!(PatternFinder::all(text, pattern), vec![6, 25]);
}
