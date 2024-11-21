#![allow(missing_docs, clippy::missing_docs_in_private_items)] // TODO: docs

use std::ops::Add;

use crate::{
    ui::{Color, NodeSide, NodeUI, SocketShape, SocketUI, TextUi},
    Id, Pos,
};

/* -------------------------------------------------------------------------- */

pub trait GraphAdapter {
    type NodeId: Id;
    type SocketId: Id;

    fn accept<'graph, V>(&'graph mut self, visitor: V)
    where
        V: GraphVisitor<'graph, Self::NodeId, Self::SocketId>;
}

pub trait NodeAdapter {
    type NodeId: Id;
    type SocketId: Id;

    fn id(&self) -> Self::NodeId;

    fn pos(&self) -> Pos;

    fn set_pos(&mut self, pos: Pos);

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
    pub ui: SocketUI,
    // TODO: make `field` generic
    pub field: Option<&'field mut f32>,
}

impl<'field, Id> SocketData<'field, Id> {
    #[inline]
    pub fn new(id: Id, side: NodeSide) -> Self {
        Self {
            id,
            ui: SocketUI::new(side, false), // TODO: remove `is_connected` from ctor and make is false by default
            field: None,
        }
    }

    // TODO: inline `SocketUI` fields into `SocketData`?

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
