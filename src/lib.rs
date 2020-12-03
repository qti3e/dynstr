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

pub const MIN_SLICE_LENGTH: usize = 16;
