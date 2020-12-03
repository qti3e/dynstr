use super::StringIterator;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// An immutable string representation with efficient memory management for heavy
/// string manipulation.
#[derive(Clone)]
pub enum DynamicString {
    Empty,
    SingleOneByteChar(u8),
    SingleTwoByteChar(u16),
    SeqOneByteString(Arc<Vec<u8>>),
    SeqTwoByteString(Arc<Vec<u16>>),
    SlicedString {
        root: Box<DynamicString>,
        start: usize,
        length: usize,
    },
    ConsString {
        first: Box<DynamicString>,
        second: Box<DynamicString>,
    },
}

impl DynamicString {
    pub fn new(data: &str) -> Self {
        match (data.len(), data.is_ascii()) {
            (0, _) => DynamicString::empty(),
            (1, true) => DynamicString::SingleOneByteChar(data.as_bytes()[0]),
            (2, false) => {
                let v: Vec<u16> = data.encode_utf16().collect();
                DynamicString::SingleTwoByteChar(v[0])
            }
            (_, true) => DynamicString::SeqOneByteString(Arc::new(data.as_bytes().to_vec())),
            (_, false) => {
                DynamicString::SeqTwoByteString(Arc::new(data.encode_utf16().into_iter().collect()))
            }
        }
    }

    /// Returns a new empty string.
    #[inline]
    pub fn empty() -> Self {
        DynamicString::Empty
    }

    /// Returns length of the string.
    pub fn len(&self) -> usize {
        match self {
            DynamicString::Empty => 0,
            DynamicString::SingleOneByteChar(_) | DynamicString::SingleTwoByteChar(_) => 1,
            DynamicString::SeqOneByteString(v) => v.len(),
            DynamicString::SeqTwoByteString(v) => v.len(),
            DynamicString::SlicedString { length, .. } => *length,
            DynamicString::ConsString { first, second } => first.len() + second.len(),
        }
    }

    /// Returns true if this string only contains one-byte characters.
    pub fn has_one_byte_char(&self) -> bool {
        match self {
            DynamicString::Empty
            | DynamicString::SingleOneByteChar(_)
            | DynamicString::SeqOneByteString(_) => true,
            DynamicString::SingleTwoByteChar(_) | DynamicString::SeqTwoByteString(_) => false,
            DynamicString::SlicedString { root, .. } => root.has_one_byte_char(),
            DynamicString::ConsString { first, second } => {
                first.has_one_byte_char() && second.has_one_byte_char()
            }
        }
    }

    #[inline(always)]
    pub fn flatten(self) -> Self {
        match &self {
            DynamicString::Empty
            | DynamicString::SingleOneByteChar(_)
            | DynamicString::SingleTwoByteChar(_)
            | DynamicString::SeqOneByteString(_)
            | DynamicString::SeqTwoByteString(_) => return self,
            _ => {}
        };

        match (self.len(), self.has_one_byte_char()) {
            (0, _) => DynamicString::empty(),
            (1, true) => {
                let bytes: Vec<u16> = self.into_iter().collect();
                DynamicString::SingleOneByteChar(bytes[0] as u8)
            }
            (1, false) => {
                let bytes: Vec<u16> = self.into_iter().collect();
                DynamicString::SingleTwoByteChar(bytes[0])
            }
            (_, true) => {
                let bytes: Vec<u8> = self.into_iter().map(|x| x as u8).collect();
                DynamicString::SeqOneByteString(Arc::new(bytes))
            }
            _ => {
                let bytes: Vec<u16> = self.into_iter().collect();
                DynamicString::SeqTwoByteString(Arc::new(bytes))
            }
        }
    }

    /// Returns an iterator over the characters in this string.
    #[inline]
    pub fn iter(&self) -> StringIterator {
        self.clone().into_iter()
    }
}

impl From<DynamicString> for String {
    fn from(str: DynamicString) -> Self {
        let vec: Vec<u16> = str.into_iter().collect();
        String::from_utf16_lossy(&vec)
    }
}

impl From<&DynamicString> for String {
    fn from(str: &DynamicString) -> Self {
        let vec: Vec<u16> = str.iter().collect();
        String::from_utf16_lossy(&vec)
    }
}

impl Hash for DynamicString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO(qti3e) It can be more efficient.
        for c in self.iter() {
            state.write_u16(c);
        }
    }
}

impl PartialEq<DynamicString> for DynamicString {
    fn eq(&self, other: &DynamicString) -> bool {
        match (self, other) {
            (DynamicString::Empty, DynamicString::Empty) => true,
            (DynamicString::Empty, _) => false,
            (DynamicString::SingleOneByteChar(v1), DynamicString::SingleOneByteChar(v2)) => *v1 == *v2,
            (DynamicString::SingleTwoByteChar(v1), DynamicString::SingleTwoByteChar(v2)) => *v1 == *v2,
            (DynamicString::SeqOneByteString(v1), DynamicString::SeqOneByteString(v2)) => v1 == v2,
            (DynamicString::SeqTwoByteString(v1), DynamicString::SeqTwoByteString(v2)) => v1 == v2,
            (s, o) => s.len() == o.len() && s.iter().eq(o.iter()),
        }
    }
}

impl Debug for DynamicString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        String::from(self).fmt(f)
    }
}
