use super::{DynamicString, IndexedString};

/// A lazy (iterative) pattern finder using KMP string searching algorithm.
#[derive(Debug)]
pub struct PatternFinder {
    text: Option<IndexedString>,
    pattern: Option<IndexedString>,
    lps_array: Option<Vec<usize>>,
    contains_zero: bool,
    // iterator state
    done: bool,
    text_index: usize,
    pattern_index: usize,
}

impl PatternFinder {
    pub fn new(text: DynamicString, pattern: DynamicString) -> Self {
        let ptn_len = pattern.len();
        let txt_len = text.len();

        match (txt_len, ptn_len) {
            (_, 0) => PatternFinder {
                text: None,
                pattern: None,
                lps_array: None,
                contains_zero: true,
                done: false,
                text_index: 0,
                pattern_index: 0,
            },
            (0, _) => PatternFinder {
                text: None,
                pattern: None,
                lps_array: None,
                contains_zero: false,
                done: false,
                text_index: 0,
                pattern_index: 0,
            },
            _ if ptn_len > txt_len => PatternFinder {
                text: None,
                pattern: None,
                lps_array: None,
                contains_zero: false,
                done: false,
                text_index: 0,
                pattern_index: 0,
            },
            _ if ptn_len == txt_len => PatternFinder {
                text: None,
                pattern: None,
                lps_array: None,
                contains_zero: text.eq(&pattern),
                done: false,
                text_index: 0,
                pattern_index: 0,
            },
            _ => PatternFinder {
                text: Some(IndexedString::new(text)),
                pattern: Some(IndexedString::new(pattern)),
                lps_array: None,
                contains_zero: false,
                done: false,
                text_index: 0,
                pattern_index: 0,
            },
        }
    }

    pub fn all(text: DynamicString, pattern: DynamicString) -> Vec<usize> {
        Self::new(text, pattern).collect()
    }
}

impl Iterator for PatternFinder {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.done {
            return None;
        }

        if self.contains_zero {
            self.done = true;
            return match self.text_index {
                0 => Some(0),
                _ => None,
            };
        }

        let text = self.text.as_ref().unwrap();
        let pattern = self.pattern.as_ref().unwrap();
        let lps = self
            .lps_array
            .get_or_insert_with(|| compute_lps_array(pattern));
        let len = text.len();
        let ptn_len = pattern.len();

        let mut i = self.text_index;
        let mut j = self.pattern_index;

        while i < len {
            if pattern.at(j) == text.at(i) {
                j += 1;
                i += 1;
            }

            if j == ptn_len {
                self.text_index = i;
                self.pattern_index = lps[j - 1];
                return Some(i - j);
            }

            if i < len && pattern.at(j) != text.at(i) {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }

        self.done = true;
        None
    }
}

#[inline(always)]
fn compute_lps_array(pattern: &IndexedString) -> Vec<usize> {
    let ptn_len = pattern.len();
    let mut lps = vec![0; ptn_len];

    // length of the previous longest prefix suffix
    let mut len = 0;
    lps[0] = 0;

    // the loop calculates lps[i] for i = 1 to ptn_len-1
    let mut i = 1;
    while i < ptn_len {
        if pattern.at(i) == pattern.at(len) {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}
