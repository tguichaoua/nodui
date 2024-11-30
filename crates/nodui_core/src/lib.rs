//! The core types of the [nodui] crate.
//!
//! [nodui]: https://crates.io/crates/nodui

pub mod ui;

/* -------------------------------------------------------------------------- */

/// A position in the graph coordinates system.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pos {
    #[allow(missing_docs)]
    pub x: i32,
    #[allow(missing_docs)]
    pub y: i32,
}

impl Pos {
    /// Creates a [`Pos`].
    #[inline]
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

/* -------------------------------------------------------------------------- */
