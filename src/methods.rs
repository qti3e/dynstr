use super::{DynamicString, PatternFinder, MIN_SLICE_LENGTH};
use std::cmp;

impl DynamicString {
    /// Extracts a section of a string and returns it as a new string, without modifying
    /// the original string.
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
    /// ```
    /// use dynstr::DynamicString;
    /// let str = DynamicString::new("hello");
    /// assert_eq!(str.append(" world"), DynamicString::new("hello world"));
    /// ```
    pub fn append<T: Into<DynamicString>>(&self, other: T) -> Self {
        let other = other.into();
        match (self, &other) {
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

    /// Return the index of the first occurrence of the specified value in the current string.
    /// ```
    /// use dynstr::DynamicString;
    /// let str = DynamicString::new("Hello world");
    /// assert_eq!(str.index_of("world"), Some(6));
    /// assert_eq!(str.index_of("world!"), None);
    /// ```
    pub fn index_of<T: Into<DynamicString>>(&self, pattern: T) -> Option<usize> {
        PatternFinder::new(self.clone(), pattern.into()).next()
    }

    /// Divides a String into an ordered list of substrings, puts these substrings into a vector,
    /// and returns the vector. The division is done by searching for a pattern; where the pattern
    /// is provided as the first parameter in the method's call.   
    /// This method tries to follow the JavaScript's String.split method in edge cases.
    /// ```
    /// use dynstr::DynamicString;
    /// let str = DynamicString::new("Hello world");
    /// assert_eq!(DynamicString::new("Jack,Joe,John").split(",", None), vec!["Jack", "Joe", "John"]);
    /// assert_eq!(DynamicString::new("Jack,Joe,John").split(",", Some(1)), vec!["Jack"]);
    /// // edge cases:
    /// assert!(DynamicString::new("").split("", None).is_empty());
    /// assert_eq!(DynamicString::new("ABC").split("", None), vec!["A", "B", "C"]);
    /// assert_eq!(DynamicString::new("").split("ABC", None), vec![""]);
    /// ```
    pub fn split<T: Into<DynamicString>>(
        &self,
        separator: T,
        limit: Option<usize>,
    ) -> Vec<DynamicString> {
        if limit == Some(0) {
            return Vec::with_capacity(0);
        }

        let separator = separator.into();
        let sep_len = separator.len();
        let patterns = PatternFinder::new(self.clone(), separator);
        let mut result = Vec::new();
        let mut last_index = 0;

        for index in patterns {
            if !(sep_len == 0 && last_index == 0 && index == 0) {
                result.push(self.slice(last_index, index - last_index));
            }
            last_index = index + sep_len;
            match limit {
                Some(n) if n == result.len() => return result,
                _ => {}
            }
        }

        if last_index < self.len() {
            result.push(self.slice(last_index, self.len() - last_index));
        }

        result
    }

    /// Determines whether a string begins with the characters of a specified string, returning
    /// true or false as appropriate.
    pub fn starts_with<T: Into<DynamicString>>(&self, other: T) -> bool {
        let o: DynamicString = other.into();
        if o.len() > self.len() {
            false
        } else {
            self.iter().take(o.len()).eq(o.iter())
        }
    }
}

impl std::ops::Add<DynamicString> for DynamicString {
    type Output = DynamicString;

    fn add(self, rhs: DynamicString) -> Self::Output {
        self.append(rhs)
    }
}
