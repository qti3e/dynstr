use super::{DynamicString, IndexedString};

/// A lazy (iterative) string matcher.
#[derive(Debug, Clone)]
pub struct PatternFinder(PatternFinderInner);

impl PatternFinder {
    /// Creates a new PatternFinder which will search for the given `pattern` in the given
    /// `text`.
    pub fn new(text: DynamicString, pattern: DynamicString) -> Self {
        let txt_len = text.len();
        let ptn_len = pattern.len();

        PatternFinder(match (txt_len, ptn_len) {
            (0, 0) => PatternFinderInner::Zero { done: false },
            (_, 0) => PatternFinderInner::Any {
                index: 0,
                end: txt_len,
            },
            (0, _) => PatternFinderInner::Zero { done: false },
            _ if ptn_len > txt_len => PatternFinderInner::Zero { done: true },
            _ if ptn_len == txt_len => PatternFinderInner::Zero {
                done: !text.eq(&pattern),
            },
            _ => PatternFinderInner::KMP(KMPPatternFinder::new(text, pattern)),
        })
    }

    /// Returns a vector containing index of all the occurrences.
    #[inline]
    pub fn all(text: DynamicString, pattern: DynamicString) -> Vec<usize> {
        Self::new(text, pattern).collect()
    }
}

impl Iterator for PatternFinder {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        self.0.next()
    }
}

#[derive(Debug, Clone)]
enum PatternFinderInner {
    /// This finder will ony yield one 0 and finish.
    Zero { done: bool },
    /// Yield all the number until the end.
    Any { index: usize, end: usize },
    /// Use KMP finder.
    KMP(KMPPatternFinder),
}

impl Iterator for PatternFinderInner {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        match self {
            PatternFinderInner::Zero { done } => {
                if *done {
                    None
                } else {
                    *done = true;
                    Some(0)
                }
            }
            PatternFinderInner::Any { index, end } => {
                if index == end {
                    None
                } else {
                    let c = *index;
                    *index += 1;
                    Some(c)
                }
            }
            PatternFinderInner::KMP(finder) => finder.next(),
        }
    }
}

#[derive(Debug, Clone)]
struct KMPPatternFinder {
    text: IndexedString,
    pattern: IndexedString,
    lps_array: Option<Vec<usize>>,
    // iterator state
    done: bool,
    text_index: usize,
    pattern_index: usize,
}

impl KMPPatternFinder {
    #[inline]
    pub fn new(text: DynamicString, pattern: DynamicString) -> Self {
        assert!(text.len() > 0);
        assert!(pattern.len() > 0);
        KMPPatternFinder {
            text: IndexedString::new(text),
            pattern: IndexedString::new(pattern),
            lps_array: None,
            done: false,
            text_index: 0,
            pattern_index: 0,
        }
    }
}

impl Iterator for KMPPatternFinder {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.done {
            return None;
        }

        let text = &self.text;
        let pattern = &self.pattern;
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
