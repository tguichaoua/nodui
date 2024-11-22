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

    /// Provides the nodes using an [`Iterator`].
    ///
    /// This method is equivalent to
    /// ```
    /// # use nodui_core::{GraphVisitor, NodeAdapter, NodeSeq, SizeHint};
    /// # fn accept<N, S>(
    /// #   mut visitor: impl GraphVisitor<'_, N, S>,
    /// #   nodes: impl Iterator<Item: NodeAdapter<NodeId = N, SocketId = S>>
    /// # ) {
    /// let mut node_seq = visitor.nodes(SizeHint::of(&nodes));
    /// for node in nodes {
    ///    node_seq.visit_node(node);
    /// }
    /// # }
    /// ```
    #[inline]
    fn nodes_iterator<I>(&mut self, nodes: I)
    where
        I: IntoIterator,
        I::Item: NodeAdapter<NodeId = NodeId, SocketId = SocketId>,
    {
        let iter = nodes.into_iter();
        let size_hint = SizeHint::of(&iter);
        let mut seq = self.nodes(size_hint);
        for node in iter {
            seq.visit_node(node);
        }
    }
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

    /// Provides the [`SocketData`] using an [`Iterator`].
    ///
    /// This method is equivalent to
    /// ```
    /// # use nodui_core::{NodeVisitor, SocketData, SocketSeq, SizeHint};
    /// # fn accept<'a, S>(
    /// #   mut visitor: impl NodeVisitor<'a, S>,
    /// #   sockets: impl Iterator<Item = SocketData<'a, S>>
    /// # ) {
    /// let mut socket_seq = visitor.sockets(SizeHint::of(&sockets));
    /// for socket in sockets {
    ///    socket_seq.visit_socket(socket);
    /// }
    /// # }
    /// ```
    #[inline]
    fn sockets_iterator<I>(&mut self, sockets: I)
    where
        I: IntoIterator<Item = SocketData<'node, SocketId>>,
    {
        let iter = sockets.into_iter();
        let size_hint = SizeHint::of(&iter);
        let mut seq = self.sockets(size_hint);
        for socket in iter {
            seq.visit_socket(socket);
        }
    }
}

/// A socket sequence used to fetch the sockets from an [`NodeAdapter`](super::NodeAdapter).
///
/// See [`NodeVisitor::sockets`].
pub trait SocketSeq<'node, SocketId> {
    /// Fetch a [`SocketData`].
    fn visit_socket(&mut self, socket: SocketData<'node, SocketId>);
}
