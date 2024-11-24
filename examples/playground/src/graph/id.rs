//! Identifier for the graph.

use serde::{Deserialize, Serialize};

/* -------------------------------------------------------------------------- */

/// The unique identifier for a node of the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId(uuid::Uuid);

impl core::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl NodeId {
    /// Generates a new random [`NodeId`].
    pub(super) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

/* -------------------------------------------------------------------------- */

/// The unique identifier for a socket of the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SocketId(uuid::Uuid);

impl core::fmt::Display for SocketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl SocketId {
    /// Generates a new random [`SocketId`].
    pub(super) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

/* -------------------------------------------------------------------------- */
