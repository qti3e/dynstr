use dynstr::DynamicString;

#[test]
fn append() {
    let s0 = DynamicString::new("Foo");
    let s1 = DynamicString::new("Bar");
    assert_eq!(s0 + s1, "FooBar");
    assert_eq!(DynamicString::empty() + DynamicString::empty(), DynamicString::empty());
    assert_eq!(DynamicString::empty() + "X", DynamicString::new("X"));
    assert_eq!(DynamicString::new("X") + DynamicString::empty(), DynamicString::new("X"));
}

#[test]
fn index_of() {
    assert_eq!(
        DynamicString::new("Hello World").index_of(DynamicString::new("ell")),
        Some(1)
    );
    assert_eq!(
        DynamicString::new("Hello World").index_of(DynamicString::new("Wor")),
        Some(6)
    );
    assert_eq!(
        DynamicString::new("Hello World").index_of(DynamicString::new("elle")),
        None
    );
    assert_eq!(
        DynamicString::new("").index_of(DynamicString::new("")),
        Some(0)
    );
    assert_eq!(
        DynamicString::new("ABC").index_of(DynamicString::new("")),
        Some(0)
    );
}

#[test]
fn split() {
    let source = DynamicString::new("01#-;23#-;45");
    let result = source.split(DynamicString::new("#-;"), None);
    assert_eq!(
        result
            .iter()
            .map(|x| String::from(x))
            .collect::<Vec<String>>(),
        vec!["01", "23", "45"]
    );
}

#[test]
fn split_empty_source() {
    let source = DynamicString::new("");
    let result = source.split(DynamicString::new("A"), None);
    assert_eq!(
        result
            .iter()
            .map(|x| String::from(x))
            .collect::<Vec<String>>(),
        vec![""]
    );
}

#[test]
fn split_empty_pattern() {
    let source = DynamicString::new("ABC");
    let result = source.split(DynamicString::new(""), None);
    assert_eq!(
        result
            .iter()
            .map(|x| String::from(x))
            .collect::<Vec<String>>(),
        vec!["A", "B", "C"]
    );
}

#[test]
fn split_empty() {
    let source = DynamicString::new("");
    let result = source.split(DynamicString::new(""), None);
    assert_eq!(
        result
            .iter()
            .map(|x| String::from(x))
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );
}
