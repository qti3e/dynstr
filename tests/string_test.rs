use dynstr::DynamicString;

#[test]
fn test_basic() {
    let str = "Hello World!";
    let ec = DynamicString::new(str);
    assert_eq!(ec.len(), 12);
    assert_eq!(ec.has_one_byte_char(), true);
    assert_eq!(&String::from(ec), str);
}

#[test]
fn test_unicode() {
    let str = "😴";
    let ec = DynamicString::new(str);
    assert_eq!(ec.len(), 2);
    assert_eq!(ec.has_one_byte_char(), false);
    assert_eq!(&String::from(ec), str);

    let str = "😴😄😃⛔🎠🚓🚇";
    let ec = DynamicString::new(str);
    assert_eq!(ec.len(), 13);
    assert_eq!(ec.has_one_byte_char(), false);
    assert_eq!(&String::from(ec), str);
}

#[test]
fn test_iter_cons() {
    let simple = Box::new(DynamicString::new("012345"));
    let cons = DynamicString::ConsString {
        first: simple.clone(),
        second: simple.clone(),
    };
    assert_eq!(&String::from(cons), "012345012345");
}

#[test]
fn test_iter_slice() {
    let simple = Box::new(DynamicString::new("0123456789"));
    let slice = DynamicString::SlicedString {
        root: simple.clone(),
        start: 0,
        length: 3,
    };
    assert_eq!(&String::from(slice), "012");

    let slice = DynamicString::SlicedString {
        root: simple.clone(),
        start: 1,
        length: 3,
    };
    assert_eq!(&String::from(slice), "123");

    let slice = DynamicString::SlicedString {
        root: simple.clone(),
        start: 7,
        length: 15,
    };
    assert_eq!(&String::from(slice), "789");
}

#[test]
fn test_iter_cons_slice() {
    let cons = Box::new(DynamicString::ConsString {
        first: Box::new(DynamicString::new("012345")),
        second: Box::new(DynamicString::new("6789a")),
    });

    // All in first half.
    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 2,
        length: 3,
    };
    assert_eq!(&String::from(slice), "234");

    // All in first half - edge.
    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 2,
        length: 4,
    };
    assert_eq!(&String::from(slice), "2345");

    // All in second part.
    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 6,
        length: 3,
    };
    assert_eq!(&String::from(slice), "678");
    // All in second part.
    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 7,
        length: 3,
    };
    assert_eq!(&String::from(slice), "789");

    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 5,
        length: 1,
    };
    assert_eq!(&String::from(slice), "5");

    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 6,
        length: 1,
    };
    assert_eq!(&String::from(slice), "6");

    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 5,
        length: 2,
    };
    assert_eq!(&String::from(slice), "56");

    let slice = DynamicString::SlicedString {
        root: cons.clone(),
        start: 2,
        length: 7,
    };
    assert_eq!(&String::from(slice), "2345678");
}

#[test]
fn test_iter_cons_cons_slice() {
    let cons = DynamicString::ConsString {
        first: Box::new(DynamicString::new("012")),
        second: Box::new(DynamicString::ConsString {
            first: Box::new(DynamicString::new("345")),
            second: Box::new(DynamicString::new("678")),
        }),
    };

    assert_eq!(&String::from(cons.clone()), "012345678");
    let root = Box::new(cons);

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 0,
        length: 2,
    };
    assert_eq!(&String::from(slice), "01");

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 2,
        length: 2,
    };
    assert_eq!(&String::from(slice), "23");

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 4,
        length: 4,
    };
    assert_eq!(&String::from(slice), "4567");
}

#[test]
fn test_iter_slice_slice() {
    // 2345678
    let root = Box::new(DynamicString::SlicedString {
        root: Box::new(DynamicString::new("0123456789")),
        start: 2,
        length: 7,
    });

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 1,
        length: 3,
    };

    // 0123456789 <- root
    //   |------
    //  (0123456)
    //    |--
    // 01[345]789
    assert_eq!(&String::from(slice), "345");

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 2,
        length: 5,
    };
    assert_eq!(&String::from(slice), "45678");

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 3,
        length: 5,
    };
    // 0123456789 <- root
    //   |------
    //  (0123456789)
    //      |---x
    // 0123[5678]
    assert_eq!(&String::from(slice), "5678");
}

#[test]
fn test_iter_slice_slice_slice() {
    let root = Box::new(DynamicString::SlicedString {
        // 0123456789
        root: Box::new(DynamicString::SlicedString {
            root: Box::new(DynamicString::new("abc0123456789def")),
            start: 3,
            length: 10,
        }),
        start: 2,
        length: 7,
    });

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 1,
        length: 3,
    };
    assert_eq!(&String::from(slice), "345");

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 2,
        length: 5,
    };
    assert_eq!(&String::from(slice), "45678");

    let slice = DynamicString::SlicedString {
        root: root.clone(),
        start: 3,
        length: 5,
    };
    assert_eq!(&String::from(slice), "5678");
}

#[test]
fn test_nth() {
    let sentence = {
        let str = "In formal language theory and computer programming, string concatenation is the operation of joining character strings end-to-end. For example, the concatenation of \"snow\" and \"ball\" is \"snowball\". Wikipedia";
        let long = Box::new(DynamicString::new(str));
        let computer = Box::new(DynamicString::SlicedString {
            root: long.clone(),
            start: 30,
            length: 8,
        });
        let is = Box::new(DynamicString::SlicedString {
            root: long.clone(),
            start: 73,
            length: 2,
        });
        let theory = Box::new(DynamicString::SlicedString {
            root: long.clone(),
            start: 19,
            length: 6,
        });

        let space = Box::new(DynamicString::new(" "));

        DynamicString::ConsString {
            first: computer,
            second: Box::new(DynamicString::ConsString {
                first: space.clone(),
                second: Box::new(DynamicString::ConsString {
                    first: is,
                    second: Box::new(DynamicString::ConsString {
                        first: space.clone(),
                        second: theory,
                    }),
                }),
            }),
        }
    };

    assert_eq!(&String::from(sentence.clone()), "computer is theory");
    assert_eq!(sentence.iter().nth(0), Some(99));
    assert_eq!(sentence.iter().nth(8), Some(32));
    assert_eq!(sentence.iter().nth(11), Some(32));
    assert_eq!(sentence.iter().nth(12), Some(116));
    assert_eq!(sentence.iter().nth(15), Some(111));
    assert_eq!(sentence.iter().nth(17), Some(121));

    {
        let mut iter = sentence.iter();
        // i = i-prev + nth + 1
        assert_eq!(iter.nth(0), Some('c' as u16)); // i = 0
        assert_eq!(iter.nth(0), Some('o' as u16)); // i = 0 + 0 + 1 = 1
        assert_eq!(iter.nth(1), Some('p' as u16)); // i = 1 + 1 + 1 = 3
        assert_eq!(iter.nth(3), Some('r' as u16)); // i = 3 + 3 + 1 = 7
        assert_eq!(iter.nth(2), Some('s' as u16)); // i = 7 + 2 + 1 = 10
    }

    {
        let ec = DynamicString::new("0123456789abcdef");
        let mut iter = ec.iter();
        assert_eq!(iter.nth(1), Some('1' as u16)); // i = 1
        assert_eq!(iter.nth(1), Some('3' as u16)); // i = 1 + 1 + 1 = 3
        assert_eq!(iter.nth(2), Some('6' as u16)); // i = 3 + 2 + 1 = 6
        assert_eq!(iter.nth(3), Some('a' as u16)); // i = 6 + 3 + 1 = 10
    }
}

#[test]
fn test_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash(ec: &DynamicString) -> u64 {
        let mut hasher = DefaultHasher::new();
        ec.hash(&mut hasher);
        hasher.finish()
    }

    assert_eq!(
        hash(&DynamicString::new("Hello")),
        hash(&DynamicString::new("Hello"))
    );
    assert_eq!(
        hash(&DynamicString::new("World")),
        hash(&DynamicString::new("World"))
    );

    assert_eq!(
        hash(&DynamicString::SlicedString {
            root: Box::new(DynamicString::new("Hello 😴")),
            start: 0,
            length: 5
        }),
        hash(&DynamicString::new("Hello"))
    );

    assert_eq!(
        hash(&DynamicString::SlicedString {
            root: Box::new(DynamicString::new("Hello 😴")),
            start: 0,
            length: 1
        }),
        hash(&DynamicString::new("H"))
    );

    assert_eq!(
        hash(&DynamicString::SlicedString {
            root: Box::new(DynamicString::new("Hello 😴")),
            start: 6,
            length: 2
        }),
        hash(&DynamicString::new("😴"))
    );
}

#[test]
fn test_eq() {
    assert_eq!(DynamicString::new("Hello"), DynamicString::new("Hello"));
    assert_ne!(DynamicString::new("Hello"), DynamicString::new("World"));

    assert_eq!(
        DynamicString::SlicedString {
            root: Box::new(DynamicString::new("Hello 😴")),
            start: 0,
            length: 5
        },
        DynamicString::new("Hello")
    );

    assert_eq!(
        DynamicString::SlicedString {
            root: Box::new(DynamicString::new("Hello 😴")),
            start: 0,
            length: 1
        },
        DynamicString::new("H")
    );

    assert_eq!(
        DynamicString::SlicedString {
            root: Box::new(DynamicString::new("Hello 😴")),
            start: 6,
            length: 2
        },
        DynamicString::new("😴")
    );
}

#[test]
fn test_index_of() {
    assert_eq!(
        DynamicString::new("Hello World").index_of(&DynamicString::new("ell")),
        Some(1)
    );
    assert_eq!(
        DynamicString::new("Hello World").index_of(&DynamicString::new("Wor")),
        Some(6)
    );
    assert_eq!(
        DynamicString::new("Hello World").index_of(&DynamicString::new("elle")),
        None
    );
}

#[test]
fn test_split() {
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
