#![allow(missing_docs, clippy::missing_docs_in_private_items)] // TODO: docs

mod socket_field;

use std::ops::Add;

use crate::{
    ui::{Color, NodeSide, NodeUI, SocketShape, SocketUI, TextUi},
    ConnectionHint, Id, Pos,
};

pub use socket_field::SocketField;

/* -------------------------------------------------------------------------- */

/// An adapter for a graph to interact with a visual editor.
pub trait GraphAdapter {
    /// An identifier used to identify a node over the graph.
    type NodeId: Id;

    /// An identifier used to identify a socket over the graph.
    type SocketId: Id;

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

    fn accept<'node, V>(&'node mut self, visitor: V)
    where
        V: NodeVisitor<'node, Self::SocketId>;
}

/* -------------------------------------------------------------------------- */

pub trait GraphVisitor<'graph, N, S> {
    fn nodes(&mut self, size_hint: SizeHint) -> impl NodeSeq<'graph, N, S>;
}

pub trait NodeVisitor<'node, S> {
    fn sockets(&mut self, size_hint: SizeHint) -> impl SocketSeq<'node, S>;
}

pub trait NodeSeq<'graph, N, S> {
    fn visit_node(&mut self, node: impl NodeAdapter<NodeId = N, SocketId = S>);
}

pub trait SocketSeq<'node, S> {
    // fn visit_socket(&mut self, id: S, ui: SocketUI, field: Option<&'node mut f32>);
    fn visit_socket(&mut self, socket: SocketData<'node, S>);
}

/* -------------------------------------------------------------------------- */

pub struct SocketData<'field, SocketId> {
    pub id: SocketId,
    /// The side of the node this socket should be placed.
    pub side: NodeSide,
    pub ui: SocketUI,
    pub field: Option<SocketField<'field>>,
}

impl<'field, Id> SocketData<'field, Id> {
    #[inline]
    pub fn new(id: Id, side: NodeSide) -> Self {
        Self {
            id,
            side,
            ui: SocketUI::default(),
            field: None,
        }
    }

    // TODO: inline `SocketUI` fields into `SocketData`?

    /// Sets the [`SocketUI`] used to render the socket.
    #[inline]
    #[must_use]
    pub fn with_ui(mut self, ui: SocketUI) -> Self {
        self.ui = ui;
        self
    }

    /// Whether the socket is connected.
    #[inline]
    #[must_use]
    pub fn with_connected(mut self, is_connected: bool) -> Self {
        self.ui.is_connected = is_connected;
        self
    }

    /// Sets the text next to the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_name(mut self, name: impl Into<TextUi>) -> Self {
        self.ui = self.ui.with_name(name);
        self
    }

    /// Sets the color of the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.ui = self.ui.with_color(color);
        self
    }

    /// Sets the shape of the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_shape(mut self, shape: SocketShape) -> Self {
        self.ui = self.ui.with_shape(shape);
        self
    }

    /// Sets the socket field.
    #[inline]
    #[must_use]
    pub fn with_field(mut self, field: impl Into<SocketField<'field>>) -> Self {
        self.field = Some(field.into());
        self
    }
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

#[derive(Debug, Clone, Copy)]
pub struct SizeHint {
    min: usize,
    max: Option<usize>,
}

impl SizeHint {
    #[must_use]
    #[inline]
    pub fn min(&self) -> usize {
        self.min
    }
    #[must_use]
    #[inline]
    pub fn max(&self) -> Option<usize> {
        self.max
    }

    #[must_use]
    #[inline]
    pub fn of<T>(x: &[T]) -> Self {
        Self::exact(x.len())
    }

    #[must_use]
    #[inline]
    pub fn of_iter<I: Iterator>(iter: &I) -> Self {
        let (min, max) = iter.size_hint();
        Self { min, max }
    }

    #[must_use]
    #[inline]
    pub fn exact(count: usize) -> Self {
        Self {
            min: count,
            max: Some(count),
        }
    }

    #[must_use]
    #[inline]
    pub fn at_least(count: usize) -> Self {
        Self {
            min: count,
            max: None,
        }
    }

    #[must_use]
    #[inline]
    pub fn at_most(count: usize) -> Self {
        Self {
            min: 0,
            max: Some(count),
        }
    }
}

impl Add for SizeHint {
    type Output = SizeHint;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        SizeHint {
            min: self.min.saturating_add(rhs.min),
            max: match (self.max, rhs.max) {
                (Some(x), Some(y)) => x.checked_add(y),
                _ => None,
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
