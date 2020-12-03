use super::DynamicString;
use std::cmp;
use std::sync::Arc;

/// The IndexedString provides an efficient random access over DynamicStrings it should be used
/// when there are frequent random accesses on the same string.
#[derive(Debug, Clone)]
pub struct IndexedString {
    chunks: Vec<(usize, Chunk)>,
    length: usize,
}

#[derive(Debug, Clone)]
enum Chunk {
    Char(u16),
    SeqOneByteString { vec: Arc<Vec<u8>>, start: usize },
    SeqTwoByteString { vec: Arc<Vec<u16>>, start: usize },
}

impl IndexedString {
    /// Creates a new IndexedString from a DynamicString.
    pub fn new(string: DynamicString) -> Self {
        let mut chunks = Vec::<(usize, Chunk)>::new();
        let mut index = 0;
        let mut to_visit = Vec::<(Box<DynamicString>, Option<usize>)>::new();
        let mut end: Option<usize> = None;
        let mut current = Box::new(string);
        let mut slice_start = 0;

        let get_len = |end: Option<usize>, len: usize| match end {
            None => len,
            Some(n) => cmp::min(n, len),
        };

        loop {
            // move current to a tmp var so that it can be modified in the match arms.
            let active = current;

            match active.as_ref() {
                DynamicString::Empty => {}
                DynamicString::SingleOneByteChar(c) => {
                    chunks.push((index, Chunk::Char(*c as u16)));
                    index += 1;
                }
                DynamicString::SingleTwoByteChar(c) => {
                    chunks.push((index, Chunk::Char(*c)));
                    index += 1;
                }
                DynamicString::SeqOneByteString(v) => {
                    let len = get_len(end, v.len()) - slice_start;
                    chunks.push((
                        index,
                        Chunk::SeqOneByteString {
                            vec: v.clone(),
                            start: slice_start,
                        },
                    ));
                    index += len;
                }
                DynamicString::SeqTwoByteString(v) => {
                    let len = get_len(end, v.len()) - slice_start;
                    chunks.push((
                        index,
                        Chunk::SeqTwoByteString {
                            vec: v.clone(),
                            start: slice_start,
                        },
                    ));
                    index += len;
                }
                DynamicString::SlicedString {
                    root,
                    start,
                    length,
                } => {
                    current = root.clone();
                    slice_start += start;
                    end = match end {
                        None => Some(slice_start + length),
                        Some(end) => Some(cmp::min(start + end, start + length)),
                    };
                    continue;
                }
                DynamicString::ConsString { first, second } => {
                    match end {
                        None => {
                            current = first.clone();
                            to_visit.push((second.clone(), None));
                        }
                        Some(n) => {
                            let first_len = first.len();
                            if first_len <= slice_start {
                                // First part is not included.
                                slice_start -= first_len;
                                end = Some(n - first_len);
                                current = second.clone();
                            } else {
                                current = first.clone();
                                if n > first_len {
                                    to_visit.push((second.clone(), Some(n - first_len)));
                                }
                            }
                        }
                    }
                    continue;
                }
            }

            match to_visit.pop() {
                None => {
                    break;
                }
                Some((chunk, e)) => {
                    current = chunk;
                    end = e;
                    slice_start = 0;
                }
            };
        }

        IndexedString {
            chunks,
            length: index,
        }
    }

    /// Return length of the string.
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Return the character at the given index.
    /// # Panics
    /// If the index is greater than or equal to the length.
    #[inline]
    pub fn at(&self, index: usize) -> u16 {
        if index >= self.length {
            panic!("Out of bound.")
        }

        match self.chunks.len() {
            1 => self.chunks[0].1.get(index),
            _ => {
                let (i, chunk) = &self.chunks[search(&self.chunks, index)];
                chunk.get(index - i)
            }
        }
    }
}

impl Chunk {
    #[inline]
    pub fn get(&self, index: usize) -> u16 {
        match self {
            Chunk::Char(c) => {
                assert_eq!(index, 0);
                *c
            }
            Chunk::SeqOneByteString { vec, start } => vec[start + index] as u16,
            Chunk::SeqTwoByteString { vec, start } => vec[start + index],
        }
    }
}

#[inline(always)]
fn search(chunks: &Vec<(usize, Chunk)>, index: usize) -> usize {
    match chunks.binary_search_by_key(&index, |&(index, _)| index) {
        Ok(n) => n,
        Err(n) => n - 1,
    }
}

impl From<DynamicString> for IndexedString {
    fn from(string: DynamicString) -> Self {
        IndexedString::new(string)
    }
}

impl From<&DynamicString> for IndexedString {
    fn from(string: &DynamicString) -> Self {
        IndexedString::new(string.clone())
    }
}
