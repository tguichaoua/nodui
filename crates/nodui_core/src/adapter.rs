//! Types and traits for interactions between the ui editor and the graph data.

use crate::ui::{NodeUI, SocketUI};

/* -------------------------------------------------------------------------- */

/// A cheap-to-clone unique identifier for the nodes and the sockets of a graph.
pub trait Id: Clone + Eq + core::hash::Hash + Send + Sync + 'static {}

impl<T> Id for T where T: Clone + Eq + core::hash::Hash + Send + Sync + 'static {}

/* -------------------------------------------------------------------------- */

/// An adapter for a graph to interact with a visual editor.
pub trait GraphAdapter {
    /// An identifier used to identify a node over the graph.
    type NodeId: Id;

    /// An identifier used to identify a socket over the graph.
    type SocketId: Id;

    /// An iterator over the node's adapters of the graph.
    fn nodes(
        &self,
    ) -> impl Iterator<Item: NodeAdapter<NodeId = Self::NodeId, SocketId = Self::SocketId>>;

    /// Set the position of a node.
    fn set_node_pos(&mut self, node_id: Self::NodeId, pos: Pos);

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

    /// An iterator over the adapters of the sockets of this node.
    fn sockets(&self) -> impl Iterator<Item: SocketAdapter<SocketId = Self::SocketId>>;

    /// The unique identifier of this node.
    fn id(&self) -> Self::NodeId;

    /// The current position of this node in the graph.
    fn pos(&self) -> Pos;

    /// Defines how the node should be rendered.
    #[inline]
    fn ui(&self) -> NodeUI {
        NodeUI::default()
    }
}

/// An adapter that represent a socket of a graph.
pub trait SocketAdapter {
    /// An identifier used to identify a socket over the graph.
    type SocketId: Id;

    /// The unique identifier of this socket.
    fn id(&self) -> Self::SocketId;

    /// Defines how the socket should be rendered.
    fn ui(&self) -> SocketUI;
}

/* -------------------------------------------------------------------------- */

#[warn(clippy::missing_trait_methods)]
impl<'a, T: GraphAdapter + ?Sized> GraphAdapter for &'a mut T {
    type NodeId = T::NodeId;
    type SocketId = T::SocketId;

    #[inline]
    fn nodes(
        &self,
    ) -> impl Iterator<Item: NodeAdapter<NodeId = Self::NodeId, SocketId = Self::SocketId>> {
        GraphAdapter::nodes(*self)
    }

    #[inline]
    fn set_node_pos(&mut self, node_id: Self::NodeId, pos: Pos) {
        GraphAdapter::set_node_pos(*self, node_id, pos);
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

/* -------------------------------------------------------------------------- */

#[warn(clippy::missing_trait_methods)]
impl<'a, T: NodeAdapter + ?Sized> NodeAdapter for &'a mut T {
    type NodeId = T::NodeId;
    type SocketId = T::SocketId;

    #[inline]
    fn sockets(&self) -> impl Iterator<Item: SocketAdapter<SocketId = Self::SocketId>> {
        NodeAdapter::sockets(*self)
    }

    #[inline]
    fn id(&self) -> Self::NodeId {
        NodeAdapter::id(*self)
    }

    #[inline]
    fn pos(&self) -> Pos {
        NodeAdapter::pos(*self)
    }

    #[inline]
    fn ui(&self) -> NodeUI {
        NodeAdapter::ui(*self)
    }
}

/* -------------------------------------------------------------------------- */

#[warn(clippy::missing_trait_methods)]
impl<'a, T: SocketAdapter + ?Sized> SocketAdapter for &'a T {
    type SocketId = T::SocketId;

    #[inline]
    fn id(&self) -> Self::SocketId {
        SocketAdapter::id(*self)
    }

    #[inline]
    fn ui(&self) -> SocketUI {
        SocketAdapter::ui(*self)
    }
}

#[warn(clippy::missing_trait_methods)]
impl<'a, T: SocketAdapter + ?Sized> SocketAdapter for &'a mut T {
    type SocketId = T::SocketId;

    #[inline]
    fn id(&self) -> Self::SocketId {
        SocketAdapter::id(*self)
    }

    #[inline]
    fn ui(&self) -> SocketUI {
        SocketAdapter::ui(*self)
    }
}

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
