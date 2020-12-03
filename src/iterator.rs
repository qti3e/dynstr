use super::DynamicString;
use std::cmp;

/// The StringIterator can be used to iterate over characters in a DynamicString.
pub struct StringIterator {
    /// The current active chunk of data that we're reading.
    active_chunk: Option<Box<DynamicString>>,
    /// Current index in the active chunk, calling `next` will return the n-th
    /// character in the `active_chunk` if it is holding n, afterwards it will
    /// advance this counter.
    chunk_index: usize,
    /// If the `active_chunk` is sliced this property will determine the bound.
    end: Option<usize>,
    /// `second` part of nested ConsStrings that we need to visit after the current
    /// chunk, the optional usize is the slice bound (i.e the next value of `end`).
    to_visit: Vec<(Box<DynamicString>, Option<usize>)>,
    /// Total number of characters in the main chunk.
    size_hint: usize,
}

impl StringIterator {
    /// Advance to the next chunk.
    #[inline(always)]
    fn advance_chunk(&mut self) {
        match self.to_visit.pop() {
            None => {
                self.active_chunk = None;
                return;
            }
            Some((chunk, end)) => {
                self.active_chunk = Some(chunk);
                self.end = end;
                self.chunk_index = 0;
            }
        };
    }

    /// Consume the current slice chunk and compute `end`.
    #[inline(always)]
    fn advance_slice_chunk(&mut self, root: &Box<DynamicString>, start: usize, length: usize) {
        self.active_chunk = Some(root.clone());
        self.chunk_index += start;
        self.end = match self.end {
            None => Some(self.chunk_index + length),
            Some(end) => Some(cmp::min(start + end, start + length)),
        };
    }

    /// Consume the current cons chunk compute `end` for the second part.
    #[inline(always)]
    fn advance_cons_chunk(&mut self, first: &Box<DynamicString>, second: &Box<DynamicString>) {
        match self.end {
            None => {
                self.active_chunk = Some(first.clone());
                self.to_visit.push((second.clone(), None));
                debug_assert!(self.chunk_index == 0);
            }
            Some(end) => {
                let first_len = first.len();
                if first_len <= self.chunk_index {
                    // First part is not included.
                    self.chunk_index -= first_len;
                    self.end = Some(end - first_len);
                    self.active_chunk = Some(second.clone());
                } else {
                    self.active_chunk = Some(first.clone());
                    if end > first_len {
                        self.to_visit.push((second.clone(), Some(end - first_len)));
                    }
                }
            }
        }
    }

    /// Returns the accessible length of the vector string by applying the
    /// value of `end`.
    #[inline(always)]
    fn actual_len(&self, len: usize) -> usize {
        match self.end {
            None => len,
            Some(n) => cmp::min(n, len),
        }
    }
}

impl Iterator for StringIterator {
    type Item = u16;

    #[inline]
    fn next(&mut self) -> Option<u16> {
        let part = match &self.active_chunk {
            None => return None,
            Some(s) => s.clone(),
        };

        let part = part.as_ref();

        match part {
            DynamicString::Empty => {
                self.advance_chunk();
                self.next()
            }
            DynamicString::SlicedString {
                root,
                start,
                length,
            } => {
                self.advance_slice_chunk(root, *start, *length);
                self.next()
            }
            DynamicString::ConsString { first, second } => {
                self.advance_cons_chunk(first, second);
                self.next()
            }
            DynamicString::SingleOneByteChar(b) => {
                self.advance_chunk();
                Some(*b as u16)
            }
            DynamicString::SingleTwoByteChar(b) => {
                self.advance_chunk();
                Some(*b as u16)
            }
            DynamicString::SeqOneByteString(vec) => {
                if self.chunk_index == self.actual_len(vec.len()) {
                    self.advance_chunk();
                    return self.next();
                }

                let byte = vec[self.chunk_index];
                self.chunk_index += 1;
                Some(byte as u16)
            }
            DynamicString::SeqTwoByteString(vec) => {
                if self.chunk_index == self.actual_len(vec.len()) {
                    self.advance_chunk();
                    return self.next();
                }

                let byte = vec[self.chunk_index];
                self.chunk_index += 1;
                Some(byte)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size_hint, Some(self.size_hint))
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        loop {
            if n == 0 {
                return self.next();
            }

            let part = match &self.active_chunk {
                None => return None,
                Some(s) => s.clone(),
            };

            let len = match part.as_ref() {
                DynamicString::Empty => {
                    self.advance_chunk();
                    continue;
                }
                DynamicString::SlicedString {
                    root,
                    start,
                    length,
                } => {
                    self.advance_slice_chunk(root, *start, *length);
                    continue;
                }
                DynamicString::ConsString { first, second } => {
                    self.advance_cons_chunk(first, second);
                    continue;
                }
                DynamicString::SingleOneByteChar(_) | DynamicString::SingleTwoByteChar(_) => {
                    n -= 1;
                    self.advance_chunk();
                    continue;
                }
                DynamicString::SeqOneByteString(v) => self.actual_len(v.len()),
                DynamicString::SeqTwoByteString(v) => self.actual_len(v.len()),
            };

            let index = self.chunk_index + n;
            if index < len {
                self.chunk_index = index + 1;
                return match part.as_ref() {
                    DynamicString::SeqOneByteString(v) => Some(v[index] as u16),
                    DynamicString::SeqTwoByteString(v) => Some(v[index]),
                    _ => unreachable!(),
                };
            }

            let rem = len - self.chunk_index;
            n -= rem;
            self.advance_chunk();
        }
    }
}

impl IntoIterator for DynamicString {
    type Item = u16;
    type IntoIter = StringIterator;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        StringIterator {
            active_chunk: Some(Box::new(self)),
            chunk_index: 0,
            end: None,
            to_visit: Vec::with_capacity(4),
            size_hint: len,
        }
    }
}
