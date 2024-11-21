//! Types and traits for interactions between the ui editor and the graph data.

/* -------------------------------------------------------------------------- */

/// A cheap-to-clone unique identifier for the nodes and the sockets of a graph.
pub trait Id: Clone + Eq + core::hash::Hash + Send + Sync + 'static {}

impl<T> Id for T where T: Clone + Eq + core::hash::Hash + Send + Sync + 'static {}

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

/// A hint that indicates if a connection could be accepted or rejected.
///
/// It's used to provide a feedback to the user before they submit a connection.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionHint {
    /// The connection is not valid.
    Reject,
    /// The connection is valid.
    Accept,
}

/* -------------------------------------------------------------------------- */
