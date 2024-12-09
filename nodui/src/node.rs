//! Rendering for nodes.

/// The layout for the body part of a node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum NodeLayout {
    /// Render the sockets into a single column.
    Single,

    /// Render the sockets into two column based of their [`NodeSide`](crate::NodeSide).
    #[default]
    Double,
}
