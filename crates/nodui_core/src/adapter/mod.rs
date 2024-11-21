//! Types and traits for interactions between the ui editor and the graph data.

mod size_hint;
mod socket_data;
mod socket_field;
mod visitor;

use crate::ui::NodeUI;

pub use size_hint::SizeHint;
pub use socket_data::SocketData;
pub use socket_field::SocketField;
pub use visitor::{GraphVisitor, NodeSeq, NodeVisitor, SocketSeq};

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

/// An adapter for a graph to interact with a visual editor.
pub trait GraphAdapter {
    /// An identifier used to identify a node over the graph.
    type NodeId: Id;

    /// An identifier used to identify a socket over the graph.
    type SocketId: Id;

    /// Accepts an [`GraphVisitor`] and provides it graph's information.
    fn accept<'graph, V>(&'graph mut self, visitor: V)
    where
        V: GraphVisitor<'graph, Self::NodeId, Self::SocketId>;

    /// A hint about the connection between the sockets `a` and `b`.
    ///
    /// This hint is used to provide a feedback to the user before they submit the connection.
    fn connection_hint(&self, a: Self::SocketId, b: Self::SocketId) -> ConnectionHint;

    /// The user submit a connection between the sockets `a` and `b`.
    fn connect(&mut self, a: Self::SocketId, b: Self::SocketId);

    /// An iterator over the connections between sockets.
    fn connections(&self) -> impl Iterator<Item = (Self::SocketId, Self::SocketId)>;
}

/// An adapter that represent a node of a graph.
pub trait NodeAdapter {
    /// An identifier used to identify a node over the graph.
    type NodeId: Id;

    /// An identifier used to identify a socket over the graph.
    type SocketId: Id;

    /// The unique identifier of this node.
    fn id(&self) -> Self::NodeId;

    /// The current position of this node in the graph.
    fn pos(&self) -> Pos;

    /// Sets the position of this node.
    fn set_pos(&mut self, pos: Pos);

    /// Defines how the node should be rendered.
    #[inline]
    fn ui(&self) -> NodeUI {
        NodeUI::default()
    }

    /// Accepts an [`NodeVisitor`] and provides it node's information.
    fn accept<'node, V>(&'node mut self, visitor: V)
    where
        V: NodeVisitor<'node, Self::SocketId>;
}

/* -------------------------------------------------------------------------- */

#[warn(clippy::missing_trait_methods)]
impl<T> GraphAdapter for &mut T
where
    T: GraphAdapter,
{
    type NodeId = T::NodeId;
    type SocketId = T::SocketId;

    #[inline]
    fn accept<'graph, V>(&'graph mut self, visitor: V)
    where
        V: GraphVisitor<'graph, Self::NodeId, Self::SocketId>,
    {
        <T as GraphAdapter>::accept(*self, visitor);
    }

    #[inline]
    fn connection_hint(&self, a: Self::SocketId, b: Self::SocketId) -> ConnectionHint {
        GraphAdapter::connection_hint(*self, a, b)
    }

    #[inline]
    fn connect(&mut self, a: Self::SocketId, b: Self::SocketId) {
        GraphAdapter::connect(*self, a, b);
    }

    #[inline]
    fn connections(&self) -> impl Iterator<Item = (Self::SocketId, Self::SocketId)> {
        GraphAdapter::connections(*self)
    }
}

#[warn(clippy::missing_trait_methods)]
impl<T> NodeAdapter for &mut T
where
    T: NodeAdapter,
{
    type NodeId = T::NodeId;
    type SocketId = T::SocketId;

    #[inline]
    fn id(&self) -> Self::NodeId {
        <T as NodeAdapter>::id(*self)
    }

    #[inline]
    fn pos(&self) -> Pos {
        <T as NodeAdapter>::pos(*self)
    }

    #[inline]
    fn set_pos(&mut self, pos: Pos) {
        <T as NodeAdapter>::set_pos(*self, pos);
    }

    #[inline]
    fn ui(&self) -> NodeUI {
        <T as NodeAdapter>::ui(*self)
    }

    #[inline]
    fn accept<'node, V>(&'node mut self, visitor: V)
    where
        V: NodeVisitor<'node, Self::SocketId>,
    {
        <T as NodeAdapter>::accept(*self, visitor);
    }
}

/* -------------------------------------------------------------------------- */
