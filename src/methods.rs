use super::{DynamicString, PatternFinder, MIN_SLICE_LENGTH};
use std::cmp;

impl DynamicString {
    /// Creates a new view over the current data.
    pub fn slice(&self, start: usize, length: usize) -> Self {
        if length == 0 {
            return DynamicString::empty();
        }

        let len = self.len();
        if start >= len {
            return DynamicString::empty();
        }

        // start < len
        // end = start + length
        // max(end) = len
        // => length = end - start
        // => max(length) = max(end) - min(start)
        //                = len - start
        // length = min(len - start, length)
        let length = cmp::min(len - start, length);

        let ret = DynamicString::SlicedString {
            root: Box::new(self.clone()),
            start,
            length,
        };

        if length < MIN_SLICE_LENGTH {
            ret.flatten()
        } else {
            ret
        }
    }

    /// Concatenate the current string with another string, returns the result.
    pub fn append(&self, other: &DynamicString) -> Self {
        match (self, other) {
            (DynamicString::Empty, _) | (_, DynamicString::Empty) => return DynamicString::empty(),
            _ => {}
        }

        let ret = DynamicString::ConsString {
            first: Box::new(self.clone()),
            second: Box::new(other.clone()),
        };

        if ret.len() < MIN_SLICE_LENGTH {
            ret.flatten()
        } else {
            ret
        }
    }

    pub fn index_of(&self, pattern: &DynamicString) -> Option<usize> {
        PatternFinder::new(self.clone(), pattern.clone()).next()
    }

    pub fn split(&self, separator: DynamicString, limit: Option<usize>) -> Vec<DynamicString> {
        if limit == Some(0) {
            return Vec::with_capacity(0);
        }

        let sep_len = separator.len();
        let patterns = PatternFinder::new(self.clone(), separator);
        let mut result = Vec::new();
        let mut last_index = 0;

        for index in patterns {
            result.push(self.slice(last_index, index - last_index));
            last_index = index + sep_len;
            match limit {
                Some(n) if n == result.len() => return result,
                _ => {}
            }
        }

        result.push(self.slice(last_index, self.len() - last_index));

        result
    }
}
