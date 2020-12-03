/*!
This crate provides an `String` implementation which is optimized for string-manipulations,
such as concatenating and slicing.

It is suited for situations where there are lots of dynamic concatenating and slicing such
as, but not limited to, Compilers, Interpreters, Template Engines and more.
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
