/*!
This crate provides an `String` implementation which is optimized for string-manipulations,
such as concatenating and slicing.

It is suited for situations where there are lots of dynamic concatenating and slicing such
as, but not limited to, Parsers, Interpreters, Template Engines and more.

# Example
Event though this example doesn't actually improve the performance (even decreases it), it
can demonstrate the basic usage of this library.
```rust
use dynstr::DynamicString;

fn main() {
    let s0 = DynamicString::new("Hello");
    let s1 = DynamicString::new("World");
    let con: DynamicString = s0 + " " + s1;
    println!("{}", con);
    let hello = con.slice(0, 5);
    assert_eq!(hello, "Hello");
}
```

Note: Any string that has less than 16 bytes is flattened.
(Gets copied instead of being referenced.)
*/

mod indexed;
mod iterator;
mod methods;
mod pattern;
mod string;

pub use indexed::*;
pub use iterator::*;
pub use methods::*;
pub use pattern::*;
pub use string::*;

pub(crate) const MIN_SLICE_LENGTH: usize = 16;
