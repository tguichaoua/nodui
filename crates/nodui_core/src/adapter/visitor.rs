//! Visitors to visit a graph.

use super::{NodeAdapter, SocketData};
use crate::SizeHint;

/// A visitor that can visit a [`GraphAdapter`](super::GraphAdapter).
///
/// The lifetime `'graph` may be used by the visitor to borrow data from the
/// [`GraphAdapter`](super::GraphAdapter).
pub trait GraphVisitor<'graph, NodeId, SocketId> {
    /// Prepares the visitor to receive the nodes.
    ///
    /// This returns a [`NodeSeq`] which will fetch the nodes.
    ///
    /// The `size_hint` indicates the number of nodes that will be provided,
    /// this may be used by the visitor to perform optimization.
    fn nodes(&mut self, size_hint: SizeHint) -> impl NodeSeq<'graph, NodeId, SocketId>;
}

/// A node sequence used to fetch the nodes from an [`GraphAdapter`](super::GraphAdapter).
///
/// See [`GraphVisitor::nodes`].
pub trait NodeSeq<'graph, NodeId, SocketId> {
    /// Fetch a [`NodeAdapter`].
    fn visit_node(&mut self, node: impl NodeAdapter<NodeId = NodeId, SocketId = SocketId>);
}

/// A visitor that can visit a [`NodeAdapter`](super::NodeAdapter).
///
/// The lifetime `'node` may be used by the visitor to borrow data from the
/// [`NodeAdapter`](super::NodeAdapter).
pub trait NodeVisitor<'node, SocketId> {
    /// Prepares the visitor to receive the sockets.
    ///
    /// This returns a [`SocketSeq`] which will fetch the sockets.
    ///
    /// The `size_hint` indicates the number of socket that will be provided,
    /// this may be used by the visitor to perform optimization.
    fn sockets(&mut self, size_hint: SizeHint) -> impl SocketSeq<'node, SocketId>;
}

/// A socket sequence used to fetch the sockets from an [`NodeAdapter`](super::NodeAdapter).
///
/// See [`NodeVisitor::sockets`].
pub trait SocketSeq<'node, SocketId> {
    /// Fetch a [`SocketData`].
    fn visit_socket(&mut self, socket: SocketData<'node, SocketId>);
}
