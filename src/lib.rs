#![warn(clippy::all, missing_docs, trivial_casts, trivial_numeric_casts)]
//! Implementations of Java classes in pure Rust
//!
//! For interop with your old Java applications or the like

/// Implentations from `java.lang.Object`
///
/// Only consists of the `HashCode` trait, since the
/// other methods don't make sense to port to Rust
pub mod object;
/// Implements of classes from `java.util`
pub mod util;

pub use crate::object::HashCode;
pub use crate::util::Random;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
